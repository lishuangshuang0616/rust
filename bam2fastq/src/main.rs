
use std::fmt::Result;
use std::fs::create_dir;
use std::fs::File;
use std::io::{BufferWriter, Write};
use std::path::{Path, PathBuf};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use anyhow::{anyhow, Context, Error};
use flate2::write::GzEncoder;
use tempfile::NamedTempFile;

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
);

#[derive(Debug, Deserialize, Clone)]
pub struct Args {
    args_bam: String,
    args_output_path: String,
    flag_nthreads: usize,
    flag_locus: Option<String>,
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
                SpecEntry::Ns(7),
                SpecEntry::Read,
            ],
            r2_spec: vec![SpecEntry::Read],
            i1_spec: vec![SpecEntry::Tags("BC".to_string(), "QT".to_string())],
            i2_spec: vec![],
            rename: None,
            order: [1, 3, 2, 0]
        }
    }

    pub fn cr11<R: bam::Read> (reader: &R) -> Self {
        Self {
            rg_spec: Self::parse_rgs(reader),
            r1_spec: vec![SpecEntry::Read],
            r2_spec: vec![SpecEntry::Tags("UR".to_string(), "UQ".to_string())],
            i1_spec: vec![SpecEntry::Tags("CR".to_string(), "CQ".to_string())],
            i2_spec: vec![SpecEntry::Tags("BC".to_string(), "QT".to_string())],
            rename: Some(vec![
                "R1".to_string(),
                "R3".to_string(),
                "R2".to_string(),
                "I1".to_string(),
            ]),
            order: [1, 3, 2, 0]
        }
    }

    fn try_get_rg(&self, rec: &Record) -> Option<Rg> {
        let rg = rec.aux(b"RG");
        match rg {
            Ok(Aux::String(s)) => {
                let key = String::from_utf8(Vec::from(s)).unwrap();
                self.rg_spec.get(&key).cloned()
            }
            Ok(..) => panic!(
                "invalid type of RG header. record: {}",
                str::from_utf8(rec.qname()).unwrap()
            ),
            Err(_) => None,
        }
    }

    pub fn find_rg(&self, rec: &Record) -> Option<Rg> {
        let main_rg_tag = self.try_get_rg(rec);

        if main_rg_tag.is_some() {
            main_rg_tag
        } else {
            let emit = |tag| {
                let corrected_bc = String::from_utf8(Vec::from(tag)).unwrap();
                let mut parts = corrected_bc.split('-');
                let _ = parts.next();
                match parts.next() {
                    Some(v) => {
                        match u32::from_str(v) {
                            Ok(v) => {
                                //println!("got gg: {}", v);
                                let name = format!("gemgroup{:03}", v);
                                self.rg_spec.get(&name).cloned()
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            };

            // Workaround for early CR 1.1 and 1.2 data
            // Attempt to extract the gem group out of the corrected barcode tag (CB)
            if let Ok(Aux::String(s)) = rec.aux(b"CB") {
                return emit(s);
            }

            // Workaround for GemCode (Long Ranger 1.3) data
            // Attempt to extract the gem group out of the corrected barcode tag (BX)
            if let Ok(Aux::String(s)) = rec.aux(b"BX") {
                return emit(s);
            }

            None
        }
    }

    fn fetch_tag(
        rec: &Record, 
        tag: &str, 
        last_tag: bool,
        dest: &mut Vec<u8>,
    ) -> Result<(), Error> {
        match rec.aux(tag.as_bytes()) {
            Ok(Aux::String(s)) => dest.extend_from_slice(s.as_bytes()),
            Ok(Aux::Char(s)) => dest.push(s),
            Err(_) => { 
                if last_tag {
                    return Ok(());
                }
                let e = anyhow!(
                    "BAM recode missing tag: {:?} on read {:?}. You do not appear to have an original 10x BAM file.\nIf you downloaded this BAM file from SRA, you likely need to download the 'Original Format' version of the BAM available for most 10x datasets.",
                    tag,
                    str::from_utf8(rec.qname()).unwrap()
                );
                return Err(e);
            }
            Ok(tag_type) => {
                let e = anyhow!(
                    "Invalid BAM record: read: {:?} unexpected tag type. Expected string for {:?}, got {:?}.\nYou do not appear to have an original 10x BAM file.If you downloaded this BAM file from SRA, you likely need to download the 'Original Format' version of the BAM available for most 10x datasets.",
                    str::from_utf8(rec.qname()).unwrap(),
                    tag,
                    tag_type
                );
                return Err(e);
            }
            
        }
        Ok(())
    }

    pub fn bam_ref_to_fq(
        &self, 
        rec: &Record, 
        spec: &[SpecEntry], 
        read_number: u32
    ) -> Result<FqRecord, Error> {
        let mut head = Vec::new();
        head.extend_from_slice(rec.qname());
        let head_suffix = format!(" {}:N:0:0", read_number);
        head.extend(head_suffix.as_bytes());
        
        let mut read = Vec::new();
        let mut qv = Vec::new();
        
        for (idx, item) in spec.iter().enumerate() {
            let last_item = idx == spec.len() -1;

            match item {
                SpecEntry::Tags (ref read_tag, ref qv_tag ) => {
                    Self::fetch_tag(rec, read_tag, last_item, &mut read)?;
                    Self::fetch_tag(rec, qv_tag, last_item, &mut qv)?;
                }

                SpecEntry::Ns(len) => {
                    for _ in 0..*len {
                        read.push(b'N');
                        qv.push(b"J");
                    }
                }

                SpecEntry::Read => {
                    let mut seq = rec.seq().as_bytes();
                    let mut qual: Vec<u8> = rec.qual().iter().map(|x| x + 33 ).collect();
                    
                    if rec.is_reverse() {
                        seq.reverse();
                        for b in seq.iter_mut() {
                            *b = complement(*b);
                        }
                        qual.reverse();
                    }

                    read.extend(seq);
                    qv.extend(qual);
                    
                }

            }
        }
        let fq_rec = FqRecord {
            head:head,
            seq: read,
            qual: qv,
        };
        
        Ok(fq_rec)    
    }


    pub fn bam_ref_to_ser(&self, rec: &Record) -> Result<SerFq, Error> {
        Ok(
            match (rec.is_first_in_template(), rec.is_last_in_template()) {
                (true, false) => SerFq {
                    header_key: rec.qname().to_vec(),
                    read_group: self.find_rg(rec),
                    read_num: ReadNum::R1,
                    rec: self
                        .bam_ref_to_fq(rec, &self.r1_spec, self.order[0])
                        .unwrap(),
                    i1: if !self.i1_spec.is_empty() {
                        Some(self.bam_ref_to_fq(rec, &self.i1_spec, self.order[2])?)
                    } else {
                        None
                    },
                    i2: if !self.i2_spec.is_empty() {
                        Some(self.bam_ref_to_fq(rec, &self.i2_spec, self.order[3])?)
                    } else {
                        None
                    },
                },
                (false, true) => SerFq {
                    header_key: rec.qname().to_vec(),
                    read_group: self.find_rg(rec),
                    read_num: ReadNum::R2,
                    rec: self
                        .bam_ref_to_fq(rec, &self.r2_spec, self.order[1])
                        .unwrap(),
                    i1: if !self.i1_spec.is_empty() {
                        Some(self.bam_ref_to_fq(rec, &self.i1_spec, self.order[2])?)
                    } else {
                        None
                    },
                    i2: if !self.i2_spec.is_empty() {
                        Some(self.bam_ref_to_fq(rec, &self.i2_spec, self.order[3])?)
                    } else {
                        None
                    },
                },
                
                _ => {
                    let e = anyhow! (
                        "Not a valid read pair: {}, {}",
                        rec.is_first_in_template(),
                        rec.is_last_in_template()
                    );
                    return Err(e);
                }
            },
        )
    }

    pub fn format_read_pair (
        &self,
        r1_rec: &Record,
        r2_rec: &Record,
    ) -> Result<FormattedReadPair, Error> {
        let r1 = self.bam_ref_to_fq(r1_rec, &self.r1_spec, self.order[0])?;
        let r2 = self.bam_ref_to_fq(r2_rec, &self.r2_spec, self.order[1])?;
        
        let i1 = if !self.i1_spec.is_empty() {
            Some(self.bam_ref_to_fq(r1_rec, &self.i1_spec, self.order[2])?)
        } else {
            None
        };
        let i2 = if !self.i2_spec.is_empty() {
            Some(self.bam_ref_to_fq(r2_rec, &self.i2_spec, self.order[3])?)
        } else {
            None
        };
        let rg = self.find_rg(r1_rec);
        Ok((rg,r1,r2,i1,i2))
    }

    pub fn format_read(
        &self, 
        rec: &Record
    ) -> Result<FormattedRead, Error> {
        let r1 = self.bam_ref_to_fq(rec, &self.r1_spec, self.order[0])?;
        let r2 = self.bam_ref_to_fq(rec, &self.r2_spec, self.order[1])?;

        let i1 = if !self.i1_spec.is_empty() {
            Some(self.bam_ref_to_fq(rec, &self.i1_spec, self.order[2])?)
        } else {
            None
        };
        
        let i2 = if !self.i2_spec.is_empty() {
            Some(self.bam_ref_to_fq(rec, &self.i2_spec, self.order[3])?)
        } else {
            None
        };

        let rg = self.find_rg(rec);
        Ok((rg,r1,r2,i1,i2))
    }

    pub fn is_double_ended(&self) -> bool {
        self.r1_spec.contains(&SpecEntry::Read) && self.r2_spec.contains(&SpecEntry::Read)
    }
}

pub fn complement(b: u8) -> u8 {
    match b {
        b'A' | b'a' => b'T',
        b'C' | b'c' => b'G',
        b'G' | b'g' => b'C',
        b'T' | b't' => b'A',
        _ => panic!("unrecognized base"),
    }
}

type Bgw = ThreadProxyWriter<BufWriter<GzEncoder<File>>>;

struct FastqWriter {
    formatter: FormatBamRecords,
    out_path: PathBuf,
    sample_name: String,
    lane: u32,

    r1: Option<Bgw>,
    r2: Option<Bgw>,
    i1: Option<Bgw>,
    i2: Option<Bgw>,

    chunk_written: usize,
    total_written: usize,
    n_chunk: usize,
    reads_per_fastq:  usize,
    path_sets: Vec<(PathBuf, PathBuf, Option<PathBuf>, Option<PathBuf>)>,
}

impl FastqWriter {
    pub fn new(
        out_path: &Path,
        formatter: FormatBamRecords,
        sample_name: String,
        lane: u32,
        reads_per_fastq: usize,
    ) -> Self {
        Self{
            formatter,
            out_path: out_path.to_path_buf(),
            sample_name,
            lane,
            r1: None,
            r2: None,
            i1: None,
            i2: None,
            chunk_written: 0,
            total_written: 0,
            n_chunk: 0,
            reads_per_fastq,
            path_sets: vec![],
        }
    }
    
    fn get_paths(
        out_path: &Path,
        sample_name: &str,
        lane: u32,
        n_files: usize,
        formatter: &FormatBamRecords,
    ) -> (PathBuf, PathBuf, Option<PathBuf>, Option<PathBuf>) {
        if formatter.rename.is_none() {
            let r1 = out_path.join(format!(
                "{}_S1_L{:03}_R1_{:03}.fastq.gz",
                sample_name,
                lane,
                n_files + 1
            ));
            
            let r2 = out_path.join(format!(
                "{}_S1_L{:03}_R2_{:03}.fastq.gz",
                sample_name,
                lane,
                n_files + 1
            ));

            let i1 = out_path.join(format!(
                "{}_S1_L{:03}_I1_{:03}.fastq.gz",
                sample_name,
                lane,
                n_files + 1
            ));

            let i2 = out_path.join(format!(
                "{}_S1_L{:03}_I2_{:03}.fastq.gz",
                sample_name,
                lane,
                n_files + 1
            ));

            (
                r1,
                r2,
                if formatter.i1_spec.is_empty() {
                    Some(i1)
                } else {
                    None
                },
                if formatter.i2_spec.is_empty() {
                    Some(i2)
                } else {
                    None
                },
            )            
        } else {
            let new_read_names = formatter.rename.as_ref().unwrap();
            
            let r1 = out_path.join(format!(
                "{}_S1_L{:03}_{}_{:03}.fastq.gz",
                sample_name,
                lane,
                new_read_names[0],
                n_files + 1
            ));

            let r2 = out_path.join(format!(
                "{}_S1_L{:03}_{}_{:03}.fastq.gz",
                sample_name,
                lane,
                new_read_names[1],
                n_files + 1
            ));

            let i1 = out_path.join(format!(
                "{}_S1_L{:03}_{}_{:03}.fastq.gz",
                sample_name,
                lane,
                new_read_names[2],
                n_files + 1
            ));

            let i2 = out_path.join(format!(
                "{}_S1_L{:03}_{}_{:03}.fastq.gz",
                sample_name,
                lane,
                new_read_names[3],
                n_files + 1
            ));

            (
                r1,
                r2,
                if formatter.i1_spec.is_empty() {
                    Some(i1)
                } else {
                    None
                },
                if formatter.i2_spec.is_empty() {
                    Some(i2)
                } else {
                    None
                },
            )
        }
        
    }

    pub fn write_rec(
        w: &mut Bgw,
        rec: &FqRecord,
    ) -> Result<(), Error> {
        w.write_all(b"@")?;
        w.write_all(&rec.head)?;
        w.write_all(b"\n")?;

        w.write_all(&rec.seq)?;
        w.write_all(b"\n+\n")?;
        w.write_all(&rec.qual)?;
        w.write_all(b"\n")?;
        Ok(())
    }

    pub fn try_writer_rec(
        w: &mut Option<Bgw>,
        rec: &Option<FqRecord>,
    ) -> Result<(), Error> {
        if let Some(ref mut w) = w {
            if let Some(rec) = rec {
                FastqWriter::write_rec(w, rec)?;
            } else{
                panic!("setup errpr");
            }
        };
        Ok(())
    }

    pub fn try_writer_rec2 (
        w: &mut Option<Bgw>,
        rec: &FqRecord
    ) -> Result<(), Error> {
        if let Some(ref mut w) = w {
            FastqWriter::write_rec(w, rec)?;
        };
        Ok(())
    }

    fn open_gzip_writer<P: AsRef<Path>> (path: P) -> ThreadProxyWriter<BufWriter<GzEncoder<File>>> {
        let file = File::create(path).unwrap();
        let gz = GzEncoder::new(file, flate2::Compression::fast());
        ThreadProxyWriter::new(BufWriter::with_capacity(1<<22 ,gz), 1<< 19)
    }


    fn cycle_writers(&mut self) -> Result<(), Error> {
        let paths = Self::get_paths(
            &self.out_path, 
            &self.sample_name, 
            self.lane, 
            self.n_chunks, 
            &self.formatter
        );
        self.r1 = Some(Self::open_gzip_writer(&paths.0));
        self.r2 = Some(Self::open_gzip_writer(&paths.1));
        self.i1 = paths.2.as_ref().map(Self::open_gzip_writer);
        self.i2 = paths.3.as_ref().map(Self::open_gzip_writer);
        self.n_chunks += 1;
        self.chunk_written = 0;
        self.path_sets.push(paths);
        
        Ok(())
    }

    fn write(
        &mut self,
        r1: &FqRecord,
        r2: &FqRecord,
        i1: &Option<FqRecord>,
        i2: &Option<FqRecord>,
    ) -> Result<(), Error> {
        if self.total_written == 0 {
            let _ = create_dir(&self.out_path);
            self.cycle_writers();
        }
        
        FastqWriter::try_writer_rec2(&mut self.r1, r1)?;
        FastqWriter::try_writer_rec2(&mut self.r2, r2)?;
        FastqWriter::try_writer_rec(&mut self.i1, i1)?;
        FastqWriter::try_writer_rec(&mut self.i2, i2)?;

        self.total_written += 1;
        self.chunk_written += 1;

        if self.chunk_written >= reads_per_fastq {
            self.cycle_writers();
        }
        Ok(())
    }
}

struct FastqManager {
    writers: HashMap<Rg, FastqWriter>,
    out_path: PathBuf,
}

impl FastqManager {
    pub fn new(
        out_path: &Path,
        formatter: FormatBamRecords,
        _sample_name: String,
        reads_per_fastq: usize,
    ) -> Self {
        let mut sample_def_paths = HashMap::new();
        let mut writers = HashiMap::new();

        for (_, &(ref _samp, lane)) in formatter.rg_spec.iter() {
            let samp = _samp.clone();
            let path = sample_def_paths.entry(samp).or_insert_with(|| {
                let suffix = _samp.replace(':', "_");
                out_path.join(suffix)
            });
            
            let writer = FastqWriter::new(
                path,
                formatter.clone(),
                "bamtofastq".to_string(),
                lane,
                reads_per_fastq,
            );

            writers.insert((_samp.clone(), lane), writer);
        }
        
        FastqManager {
            writers,
            out_path: out_path.to_path_buf(),
        }
    }

    pub fn write(
        &mut self,
        rg: &Option<Rg>,
        r1: &FqRecord,
        r2: &FqRecord,
        i1: &Option<FqRecord>,
        i2: &Option<FqRecord>,
    ) -> Result<(), Error> {
        if let &Some(ref rg) = rg {
            self.writers.get_mut(rg).map(|w| w.write(r1, r2, i1, i2));
        }
        Ok(())
    }
    
    pub fn total_written(&self) -> usize {
        self.writers.iter().map(|(_, w)| w.total_written()).sum()
    }

    pub fn paths(&self) -> Vec<(PathBuf, PathBuf, Option<PathBuf>, Option<PathBuf>)> {
        self.writers.iter().flat_map(|(_, w)| w.path_sets.clone()).collect()
    }
}

fn set_panic_handler() {
    panic::set_hook(Box::new(move |info| {
        let backtrace = backtrace::Backtrace::new();
        
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let msg = match info.location() {
            Some(loc) => format!(
                "bamtofastq failed unexpectedly. Please contact support@10xgenomics.com with the following information: '{}' {}:{}:\n{:?}",
                msg, loc.file(), loc.line(), backtrace ),
            None => format!(
                "bamtofastq failed unexpectedly. Please contact support@10xgenomics.com with the following information: '{}':\n{:?}", 
                msg, backtrace ),
        };
        println!("{}", msg);
    }));
}

pub fn go(
    args: Args,
    cache_size: Option<usize>,
) -> Result<Vec<OutPaths>, Error> {
    let cache_size = cache_size.unwrap_or(500000);
    
    let path = std::path::PathBuf::from(args.arg_bam.clone());
    if !path.exists() {
        return Err(anyhow!("BAM file does not exist: {:?}", path));
    }
    
    match args.flag_locus {
        Some(ref locus) => {
            let loc = locus::Locus::from_str(locus)
                .context("Invalid locus argument. Please specify a locus in the format chr:start-end")?;
            let mut bam = bam::IndexedReader::from_path(&args.arg_bam)
                .context("Failed to open BAM file. The BAM file must be indexed when using --locus",)?;
            
            let tid = bam
                .header()
                .tid(loc.chrom.as_bytes())
                .ok_or_else(|| anyhow!("Chromosome not found in BAM header: {}", loc.chrom))?;
            
            bam.fetch((tid, loc.start, loc.end))?;
            inner(args.clone(), cache_size, bam)
        }
        None => {
            let bam = bam::Reader::from_path(&args.arg_bam)
                .context("Error opening BAM file")?;

            inner(args, cache_size, bam)
        }       
    }
}

pub fn inner<R: bam::Read>(
    args: Args,
    cache_size: usize,
    mut bam: R,
) -> Result<Vec<OutPaths>, Error> {
    bam.set_threads(args.flag_nthreads)?;
    
    let formatter = {
        let header_fmt = FormatBamRecords::from_headers(&bam);
        match header_fmt {
            Some(mut f) => {
                if f.r1_spec == vec![SpecEntry::Read]
                    && f.i1_spec == vec![SpecEntry::Tags("CR".to_string(), "CY".to_string())]
                    && f.i2_spec == vec![SpecEntry::Tags("UR".to_string(), "UY".to_string())]
                {
                    f.rename = Some(vec![
                        "R1".to_string(),
                        "R3".to_string(),
                        "R2".to_string(),
                        "I1".to_string(),
                    ])
                }

                if args.flag_gemcode {
                    return Err(anyhow!("Do not use a pipeline-specific command-line flag: --gemcode. Supplied BAM file already contains bamtofastq headers."));
                }

                if args.flag_lr20 {
                    return Err(anyhow!("Do not use a pipeline-specific command-line flag: --lr20. Supplied BAM file already contains bamtofastq headers."));
                }

                if args.flag_cr11 {
                    return Err(anyhow!("Do not use a pipeline-specific command-line flag: --cr11. Supplied BAM file already contains bamtofastq headers."));
                }
                
                f
            }            
            None => {
                if args.flag_gemcode {
                    FormatBamRecords::gemcode(&bam)
                } else if args.flag_lr20 {
                    FormatBamRecords::lr20(&bam)
                } else if args.flag_cr11 {
                    FormatBamRecords::cr11(&bam)
                } else {
                    println!("Unrecognized 10x BAM file. For BAM files produced by older pipelines, use one of the following flags:");
                    println!("--gemcode   BAM files created with GemCode data using Longranger 1.0 - 1.3");
                    println!("--lr20      BAM files created with Longranger 2.0 using Chromium Genome data");
                    println!("--cr11      BAM files created with Cell Ranger 1.0-1.1 using Single Cell 3' v1 data");
                    return Ok(vec![]);
                }
            }
        }
    };
    
    let out_path = Path::new(&args.args_output_path);
    create_dir(&args.args_output_path).context(anyhow!(
        "Failed to create output directory: {:?}. Please ensure that the directory exists and is writable.",
        &out_path
    ))?;

    let fq = FastqManager::new(
        out_path,
        formatter.clone(),
        "bamtofastq".to_string(),
        args.flag_reads_per_fastq
    );
    
    if formatter.is_double_ended() {
        proc_double_ended(
            bam.records(),
            formatter,
            fq,
            cache_size,
            args.flag_locus.is_some(),
            args.flag_relaxed,
        );
    } else {
        proc_single_ended(
            bam.records(),
            formatter,
            fq,
        )
    }
    
}

fn proc_double_ended<I, E> (
    records: I,
    formatter: FormatBamRecords,
    mut fq: FastqManager,
    cache_size: usize,
    locus: bool,
    relaxed: bool,
) -> Result<Vec<OutPaths>, Error>
where
    I: Iterator<Item = Result<Record, E>>,
    Result<Record, E>: Context<Record, E>,
{
    let tmp_file = NameTempFile::new_in(&fq.out_path)?;
    
    let total_read_pairs =  {
        let mut rp_cache = RpCache::new(cache_size, relaxed);
        
        let w: ShardWriter<SerFq, SerFqSort> = 
        
    }
    
}
