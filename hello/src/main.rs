use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use bio::io::fastq::{Reader, Record, Writer};
use clap::{App, Arg};
use flate2::bufread::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use rayon::ThreadPoolBuilder;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() -> io::Result<()> {
    let matches = App::new("Fastq Region Extractor")
        .version("0.1.1")
        .author("lishuangshuang3 <lishuangshuang3@mgi-tech.com>")
        .about("Extracts regions from FASTQ sequences")
        .arg(
            Arg::with_name("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input FASTQ file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output FASTQ file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("regions")
                .short('r')
                .long("regions")
                .value_name("REGIONS")
                .help("Comma-separated list of regions to extract (e.g., 7:16,23:32,38:47)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("threads")
                .short('t')
                .long("threads")
                .value_name("THREADS")
                .help("Number of threads to use for parallel processing")
                .takes_value(true)
        )
        .get_matches();

    let input_path = Path::new(matches.value_of("input").unwrap());
    let output_path = Path::new(matches.value_of("output").unwrap());

    // Parse regions from the command line argument
    let regions: Vec<(usize, usize)> = matches.value_of("regions").unwrap()
        .split(',')
        .map(|r| {
            let mut parts = r.split(':');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let threads: usize = matches
        .value_of("threads")
        .unwrap_or("4") // Default to 4 threads if not specified
        .parse()
        .expect("Invalid number of threads");

    ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .expect("Could not build thread pool");

    // Create a GzEncoder for writing the output in gzip format
    let compress_output = output_path.extension().and_then(|s| s.to_str()) == Some("gz");
    let temp_output_path = if compress_output {
        output_path.with_extension("")
    } else {
        output_path.to_path_buf()
    };

    let output_file = File::create(&temp_output_path)?;
    let writer = BufWriter::new(output_file);


    // Process the FASTQ file
    let records = if input_path.extension().and_then(|s| s.to_str()) == Some("gz") {
        let file = File::open(input_path)?;
        let gz_decoder = MultiGzDecoder::new(BufReader::new(file));
        let reader = Reader::new(gz_decoder);
        process_fastq(reader, &regions)
    } else {
        let file = File::open(input_path)?;
        let reader = Reader::new(BufReader::new(file));
        process_fastq(reader, &regions)
    };

    write_records(writer, records)?;

    if compress_output {
        compress_file(&temp_output_path, output_path)?;
        fs::remove_file(&temp_output_path)?;
    }

    Ok(())
}

fn process_fastq<R: io::Read + BufRead + Send + 'static>(
    reader: Reader<R>,
    regions: &[(usize, usize)],
) -> Vec<Record> {
    reader
        .records()
        .par_bridge()
        .map(|result| {
            let record = result.expect("Error reading record");
            let seq = record.seq();
            let mut new_seq = Vec::new();
            let mut new_qual = Vec::new();

            for &(start, end) in regions {
                if start < seq.len() && end <= seq.len() {
                    new_seq.extend_from_slice(&seq[start - 1..end]);
                    new_qual.extend_from_slice(&record.qual()[start - 1..end]);
                }
            }

            let new_id = record.id().trim();

            Record::with_attrs(new_id, Some(record.desc().unwrap_or("")), &new_seq, &new_qual)
        })
        .collect()
}

fn write_records(writer: BufWriter<File>, records: Vec<Record>) -> io::Result<()> {
    let mut fastq_writer = Writer::new(writer);
    for record in records {
        fastq_writer.write_record(&record)?;
    }
    fastq_writer.flush()?;
    Ok(())
}

fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    let mut encoder = GzEncoder::new(writer, Compression::default());
    io::copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}