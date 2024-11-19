
use std::fs::create_dir;
use std::fs::File;
use std::io::{BufferWriter, Write};
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use regex::Regex;

use rust_htslib::bam::record::{Aux, Record};
use rust_htslib::bam::{self, Read};

use shardio::helper::ThreadProxyWriter;
use shardio::SortKey;
use shardio::{ShardWriter, ShardReader};

use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
10x Genomics BAM to FASTQ converter.

    Tool for converting 10x BAMs produced by Cell Ranger or Long Ranger back to
    FASTQ files that can be used as inputs to re-run analysis. The FASTQ files
    emitted by the tool should contain the same set of sequences that were
    input to the original pipeline run, although the order will not be 
    preserved.  The FASTQs will be emitted into a directory structure that is
    compatible with the directories created by the 'mkfastq' tool.

    10x BAMs produced by Long Ranger v2.1+ and Cell Ranger v1.2+ contain header
    fields that permit automatic conversion to the correct FASTQ sequences.

    Older 10x pipelines require one of the arguments listed below to indicate 
    which pipeline created the BAM.

    NOTE: BAMs created by non-10x pipelines are unlikely to work correctly,
    unless all the relevant tags have been recreated.

    NOTE: BAM produced by the BASIC and ALIGNER pipeline from Long Ranger 2.1.2 and earlier
    are not compatible with bamtofastq

    NOTE: BAM files created by CR < 1.3 do not have @RG headers, so bamtofastq will use the GEM well
    annotations attached to the CB (cell barcode) tag to split data from multiple input libraries.
    Reads without a valid barcode do not carry the CB tag and will be dropped. These reads would 
    not be included in any valid cell.

Usage:
    bamtofastq [options] <bam> <output-path>
    bamtofastq (-h | --help)
  
  Options:
  
    --nthreads=<n>        Threads to use for reading BAM file [default: 4]
    --locus=<locus>       Optional. Only include read pairs mapping to locus. Use chrom:start-end format.
    --reads-per-fastq=N   Number of reads per FASTQ chunk [default: 50000000]
    --relaxed             Skip unpaired or duplicated reads instead of throwing an error.
    --gemcode             Convert a BAM produced from GemCode data (Longranger 1.0 - 1.3)
    --lr20                Convert a BAM produced by Longranger 2.0
    --cr11                Convert a BAM produced by Cell Ranger 1.0-1.1
    --bx-list=L           Only include BX values listed in text file L. Requires BX-sorted and index BAM file (see Long Ranger support for details).
    --traceback           Print full traceback if an error occurs.
    -h --help             Show this screen.
  
  ";

/*
== Dev Notes ==
This code has bunch of special cases to workaround deficiences of the BAM files produced by older pipelines,
before we started enforcing the notion that BAM files should be easily convertible back to the original
FASTQ data that was input.  Once these older pipelines & chemistries are out of service, the code could
be made considerably simpler.

1) Workaround for CR < 1.3:  there are no RG headers or RG tags, so we can't accurately get back to
per-Gem Group FASTQs, which is important because multi-gem-group experiments are common.  If we don't
have RG headers, we will set up files for 20 gem groups, and use the gem-group suffix on the CB tag to
determine the Gem group.  Reads without a CB tag will get dropped.
*/

type OutPaths = (
    PathBuf, 
    PathBuf, 
    Option<PathBuf>, 
    Option<PathBuf>
);

type FormattedReadPair = (
    Option<Rg>,
    FqRecord,
    FqRecord,
    Option<FqRecord>,
    Option<FqRecord>,
)

#[derive(Debug, Deserialize, Clone)]
pub struct Args {
    args_bam: String,
    args_output_path: String,
    flag_nthreads: usize,
    flag_locus: Option<String>,
    flag_bx_list: Option<String>,
    flag_reads_per_fastq: usize,
    flag_gemcode: bool,
    flag_lr20: bool,
    flag_cr11: bool,
    flag_traceback: bool,
    flag_relaxed: bool,
}


/// A Fastq record ready to be written to disk
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct FqRecord {
    #[serde(with="serde_bytes")]
    head: Vec<u8>,
    #[serde(with="serde_bytes")]
    seq: Vec<u8>,
    #[serde(with="serde_bytes")]
    qual: Vec<u8>,
}


#[derive()]
enum ReadNum {
    R1,
    R2,
}

// Internally serialized read
#[derive()]
struct SerFq {
    read_group: Option<Rg>,
    #[serde(with="serde_bytes")]
    header_key: Vec<u8>,
    rec: FqRecord,
    read_num: ReadNum,
    i1: Option<FqRecord>,
    i2: Option<FqRecord>,    
}

struct SerFqSort;

impl SortKey<SerFq> for SerFqSort {
    type Key = Vec<u8>;

    fn sort_key(t: &SerFq) -> Cow<Vec<u8>> {
        Cow::Borrowed(&t.header_key)
    }
}


/// Entry in the conversion spec from a BAM record back to a read.
/// Each read can be composed of data from a pair of tags (tag w/ sequence, tag w/ qual),
/// or a fixed-length sequence of Ns (with a default QV), or the sequence in the read.
enum SpecEntry {
    Tags(String, String),
    Ns(usize),
    Read,
}

type Rg = (String, U32);

#[derive(Debug, Clone)]
struct FormatBamRecords {
    rg_spec: HashMap<String, Rg>,
    r1_spec: Vec<SpecEntry>,
    r2_spec: Vec<SpecEntry>,
    i1_spec: Vec<SpecEntry>,
    i2_spec: Vec<SpecEntry>,
    rename: Option<Vec<String>>,
    order: [u32; 4]
}

impl FormatBamRecords {
    pub fn from_headers<R: bam::Read> (reader: &R) -> Option<Self> {
        let mut spec = Self::parse_spec(reader);
        let rgs = Self::parse_rgs(reader);
        let seq_names = Self::parse_seq_names(reader);
        
        if spec.is_none() {
            None
        } else {
            Some(FormatBamRecords{
                rg_spec: rgs,
                r1_spec: spec.remove("R1").unwrap(),
                r2_spec: spec.remove("R2").unwrap(),
                i1_spec: spec.remove("I1").unwrap_or_default(),
                i2_spec: spec.remove("I2").unwrap_or_default(),
                rename: seq_names,
                order: [1,3,2,4],
            })
        }
    }
    
    fn parse_spec<R: bam::Read> (reader: &R) -> HashMap<String, Vec<SpecEntry>> {
        let re = Regex::new(r"@CO\t10X_bam_to_fastq:(\S+)\((\S+)\)").unwrap();
        let text = String::from_utf8(Vec::from(reader.header().as_bytes())).unwrap();
        
        text.lines()
            .into_iter()
            .filter_map(|l|{
                re.captures(l).map(|c|{
                    let read = c.get(1).unwrap().as_str().to_string();
                    let tag_list = c.get(2).unwrap().as_str();
                    
                    let spec_entries = tag_list
                        .split(',')
                        .into_iter()
                        .map(|s|{
                            if s == "SEQ:QUAL" {
                                SpecEntry::Read
                            } else {
                                let (tag, val) = s
                                    .split(':')
                                    .map(ToString::to_string)
                                    .next_tuple()
                                    .unwrap();
                                SpecEntry::Tags(tag, val)
                            }
                        })
                        .collect();
                    (read, spec_entries)
                })
            })
            .collect()
    }
    
    fn parse_rgs<R: bam::Read> (reader: &R) -> HashMap<String, Rg> {
        let text = std::str::from_utf8(reader.header().as_bytes()).unwrap();
        let mut rg_items = text
            .lines()
            .filter(|l| l.starts_with("@RG"))
            .filter_map(Self::parse_rg_line)
            .collect::<HashMap<_,_>>();

        if rg_items.is_empty() {
            println!("WARNING: No @RG header lines found in BAM file header.");
            println!("Splitting data by the GEM well marked in the corrected barcode tag.");
            println!("Read without a corrected barcode will not appear in output FASTQs.");
        
            for i in 1..100 {
                let name = format!("gemgroup{:03}", i);
                rg_items.insert(name.clone(), (name, 0));
            }
        }
        
        rg_items
    }

    fn parse_rg_line(line: &str) -> Option<(String, (String, u32))> {
        let mut entries = line.split('\t');
        entries.next()?;
        
        let mut tags = entries
            .map(|s|s.split_once(':').unwrap())
            .collect::<HashMap<_,_>>();

        let v = tags.remove("ID")?;
        let (rg, lane) = v.rsplit_once(':').unwrap();
        
        match u32::from_str(lane) {
            Ok(lane) => Some((v.to_string(), (rg.to_string(), lane))),
            Err(_) => {
                let re = Regex::new(r"^([0-9]+)-[0-9A-F]+$").unwrap();
                let cap = re.captures(lane)?;
                let lane_u32 = u32::from_str(cap.get(1).unwrap().as_str()).unwrap();
                Some((v.to_string(), (rg.to_string(), lane_u32)))
            }
        }
    }

    fn parse_seq_names<R: bam::Read>(reader: &R) -> Option<Vec<String>> {
        let text = String::from_utf8(Vec::from(reader.header().as_bytes())).unwrap();
        let re = Regex::new(r"@CO\t10X_bam_to_fastq_seqnames:(\S+)").unwrap();

        for l in text.lines() {
            if let Some(c) = re.captures(l) {
                let names = c.get(1).unwrap().as_str().split(',');
                let seq_names = names
                    .into_iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                return Some(seq_names);
            }
        }
        None
    }

    pub fn gemcode<R: bam::Read>(reader: &R) -> Self {
        Self {
            rg_spec: Self::parse_rgs(reader),
            r1_spec: vec![SpecEntry::Read],
            r2_spec: vec![SpecEntry::Read],
            i1_spec: vec![SpecEntry::Tags("BC".to_string(), "QT".to_string())],
            i2_spec: vec![SpecEntry::Tags("RX".to_string(), "QX".to_string())],
            rename: Some(
                vec![
                    "R1".to_string(),
                    "R3".to_string(),
                    "I1".to_string(),
                    "R2".to_string(),
                ]
            ),
            order: [1,4,2,3],
        }
    }

    pub fn lr20<R: bam::Read> (reader: &R) -> Self {
        Self {
            rg_spec: Self::parse_rgs(reader),
            r1_spec: vec![
                SpecEntry::Tags("RX".to_string(), "QX".to_string()),
                SpecEntry::Ns(7);
            ],
            r2_spec: vec![SpecEntry::Read],
            i1_spec: vec![]
        }
    }


    

};


pub fn complement(b: u8) -> u8 {
    match b {
        b'A' | b'a' => b'T',
        b'C' | b'c' => b'G',
        b'G' | b'g' => b'C',
        b'T' | b't' => b'A',
        _ => panic!("unrecognized base"),
    }
}

