use clap::{Arg, Command};
use rust_htslib::bam::{self, Read};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Mutex;

fn read_mapping_file<P: AsRef<Path>>(file_path: P) -> io::Result<HashMap<String, String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(cb), Some(db)) = (parts.next(), parts.next()) {
            map.insert(cb.to_string(), db.to_string());
        }
    }
    Ok(map)
}

fn process_bam_file<P: AsRef<Path>>(
    bam_path: P,
    output_path: P,
    mapping: &HashMap<String, String>,
    threads: usize,
) -> io::Result<()> {
    let mut bam = bam::Reader::from_path(&bam_path)?;
    let header = bam::Header::from_template(bam.header());
    let mut bam_writer = bam::Writer::from_path(output_path, &header, bam::Format::BAM)?;
    
    // Set number of threads
    rust_htslib::bam::Record::set_thread_pool(num_threads)?;

    // Using a Mutex to allow safe concurrent writes to the BAM writer
    let bam_writer = Mutex::new(bam_writer);
    
    bam.records()
        .par_bridge()
        .for_each(|record| {
            let mut record = record.unwrap();
            if let Ok(cb) = record.aux(b"CB") {
                if let Some(db) = mapping.get(cb.string()) {
                    record.push_aux(b"DB", bam::record::Aux::String(db)).unwrap();
                }
            }
            bam_writer.lock().unwrap().write(&record).unwrap();
        });

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("BAM Tag Editor")
        .version("0.1.0")
        .author("Your Name <youremail@example.com>")
        .about("Adds DB tags to BAM file reads based on CB-DB mapping")
        .arg(
            Arg::new("mapping")
                .short('m')
                .long("mapping")
                .value_name("FILE")
                .help("CB-DB mapping file")
                .required(true),
        )
        .arg(
            Arg::new("bam")
                .short('b')
                .long("bam")
                .value_name("FILE")
                .help("Input BAM file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output BAM file")
                .required(true),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("N")
                .help("Number of threads")
                .default_value("4"),
        )
        .get_matches();

    let mapping_file = matches.value_of("mapping").unwrap();
    let bam_file = matches.value_of("bam").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let threads = matches.value_of_t("threads").unwrap_or(4);

    let mapping = read_mapping_file(mapping_file)?;
    process_bam_file(bam_file, output_file, &mapping, threads)?;

    Ok(())
}
