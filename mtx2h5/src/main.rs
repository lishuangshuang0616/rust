use clap::Parser;
use csv::ReaderBuilder;
use flate2::read::GzDecoder;
use hdf5::{File, types::VarLenUnicode};
use ndarray::{Array2, Axis};
use std::fs::File as StdFile;
use std::io::{BufRead, BufReader};
use std::path::Path;


/// Command line arguments
#[derive(Parser, Debug)]
#[clap(
    name = "mtx_to_h5",
    version = "0.1.0",
    author = "lishuangshuang3@mgi-tech.com",
    about = "Convert mtx format matrix to HDF5"
)]
struct Args {
    #[clap(short, long, help = "Output directory")]
    outdir: String,

    #[clap(short, long, help = "Path to matrix dir")]
    matrix: String,

    #[clap(short, long, default_value = "1", help = "Column index for gene names in feature file")]
    column: usize,

    #[clap(short, long, default_value = "GRCh38", help = "Genome name")]
    genome: String,

    #[clap(short, long, default_value = "Peak", help = "Data type (Peak or Gene)")]
    datatype: String,
}

fn read_10x_mtx<P: AsRef<Path>>(matrix_file: P, feature_file: P, barcode_file: P, datatype: &str, gene_column: usize) -> Result<(Array2<f32>, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    // Read matrix
    let matrix = read_mtx(matrix_file)?;

    // Read features
    let features = read_features(feature_file, datatype, gene_column)?;

    // Read barcodes
    let barcodes = read_barcodes(barcode_file)?;

    Ok((matrix, features, barcodes))
}

// fn read_mtx<P: AsRef<Path>>(matrix_file: P) -> Result<Array2<f32>, Box<dyn std::error::Error>> {
//     // Read the matrix file using flate2 and csv crates
//     let mut reader = BufReader::new(GzDecoder::new(StdFile::open(matrix_file)?));
//     let mut line = String::new();

//     // Skip all the comment lines that start with '%'
//     while reader.read_line(&mut line)? > 0 {
//         if !line.starts_with('%') {
//             break;
//         }
//         line.clear();
//     }

//     // Now, the current line is the summary line, so skip it
//     line.clear();
//     reader.read_line(&mut line)?;

//     // Create structures to hold matrix data
//     let mut rows = Vec::new();
//     let mut cols = Vec::new();
//     let mut data = Vec::new();

//     // Read the remaining data lines
//     for result in reader.lines() {
//         let line = result?;

//         println!("Raw line: {}", line);

//         let parts: Vec<&str> = line.split_whitespace().collect();

//         let row: usize = parts[0].parse()?;
//         let col: usize = parts[1].parse()?;
//         let value: f32 = parts[2].parse()?;

//         rows.push(row - 1); // Convert to zero-based index
//         cols.push(col - 1); // Convert to zero-based index
//         data.push(value);
//     }

//     // Create the matrix using ndarray
//     let nrows = *rows.iter().max().unwrap() + 1;
//     let ncols = *cols.iter().max().unwrap() + 1;
//     let mut matrix = Array2::<f32>::zeros((nrows, ncols));
//     for ((r, c), v) in rows.iter().zip(cols.iter()).zip(data.iter()) {
//         matrix[(*r, *c)] = *v;
//     }

//     Ok(matrix)
// }

fn read_mtx<P: AsRef<Path>>(matrix_file: P) -> Array2<f32> {
    let file = StdFile::open(matrix_file).expect("Cannot open matrix file");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let mut rows = Vec::new();
    let mut cols = Vec::new();
    let mut data = Vec::new();

    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b' ')
        .has_headers(false)
        .from_reader(reader);

    // 跳过注释行
    let mut records = csv_reader.records().skip_while(|result| {
        if let Ok(record) = result {
            record[0].starts_with('%')
        } else {
            false
        }
    });

    // 跳过总结行
    records.next();

    for result in records {
        let record = result.expect("Failed to read record");
        let row: usize = record[0].parse().expect("Failed to parse row");
        let col: usize = record[1].parse().expect("Failed to parse col");
        let value: f32 = record[2].parse().expect("Failed to parse value");

        rows.push(row - 1);
        cols.push(col - 1);
        data.push(value);
    }

    let nrows = *rows.iter().max().unwrap() + 1;
    let ncols = *cols.iter().max().unwrap() + 1;
    let mut matrix = Array2::<f32>::zeros((nrows, ncols));
    for (i, ((r, c), v)) in rows.iter().zip(cols.iter()).zip(data.iter()).enumerate() {
        matrix[(*r, *c)] = *v;
    }

    matrix
}

fn read_features<P: AsRef<Path>>(feature_file: P, datatype: &str, gene_column: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let reader: Box<dyn BufRead> = if feature_file.as_ref().ends_with(".gz") {
        Box::new(BufReader::new(GzDecoder::new(StdFile::open(feature_file)?)))
    } else {
        Box::new(BufReader::new(StdFile::open(feature_file)?))
    };

    let features = reader
        .lines()
        .map(|line| {
            let line = line?;
            if datatype == "Peak" {
                let parts: Vec<&str> = line.split('\t').collect();
                Ok::<String, Box<dyn std::error::Error>>(format!("{}:{}-{}", parts[0], parts[1], parts[2]))
            } else {
                Ok(line.split('\t')
                    .nth(gene_column - 1)
                    .ok_or("Failed to split line")?
                    .to_string())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(features)
}

fn read_barcodes<P: AsRef<Path>>(barcode_file: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let reader: Box<dyn BufRead> = if barcode_file.as_ref().ends_with(".gz") {
        Box::new(BufReader::new(GzDecoder::new(StdFile::open(barcode_file)?)))
    } else {
        Box::new(BufReader::new(StdFile::open(barcode_file)?))
    };

    let barcodes = reader
        .lines()
        .map(|line| {
            line.map(|l| l.split('\t').next().unwrap().to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(barcodes)
}

fn write_10x_h5<P: AsRef<Path>>(filename: P, matrix: Array2<f32>, features: Vec<String>, barcodes: Vec<String>, genome: &str, datatype: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    let mat_group = file.create_group("matrix")?;

    let barcode_data: Vec<VarLenUnicode> = barcodes.iter()
        .map(|s| {  
        unsafe {  
            VarLenUnicode::from_str_unchecked(s.as_str())  
        }  
        })
        .collect();
    mat_group.new_dataset_builder()
        .with_data(&barcode_data)
        .create("barcodes")?;

    let (data, indices, indptr) = sparse_matrix_to_csc(&matrix);
    mat_group.new_dataset_builder()
        .with_data(&data)
        .create("data")?;
    mat_group.new_dataset_builder()
        .with_data(&indices)
        .create("indices")?;
    mat_group.new_dataset_builder()
        .with_data(&indptr)
        .create("indptr")?;
    mat_group.new_dataset_builder()
        .with_data(&[matrix.shape()[0], matrix.shape()[1]])
        .create("shape")?;

    let features_group = file.create_group("features")?;

    let features_bytes: Vec<VarLenUnicode> = features.iter()
        .map(|s| {  
        unsafe {  
            VarLenUnicode::from_str_unchecked(s.as_str())  
        }  
        })
        .collect();
    features_group.new_dataset_builder()
        .with_data(&features_bytes)
        .create("features")?;

    let genome_bytes = unsafe {  
        VarLenUnicode::from_str_unchecked(genome)  
    };  
    features_group.new_dataset_builder()
        .with_data(&[genome_bytes.clone()])
        .create("_all_tag_keys")?;

    let datatype_bytes = unsafe {  
        VarLenUnicode::from_str_unchecked(datatype)  
    };
    features_group.new_dataset_builder()
        .with_data(&vec![datatype_bytes; features.len()])
        .create("feature_type")?;

    features_group.new_dataset_builder()
        .with_data(&[genome_bytes])
        .create("genome")?;
    features_group.new_dataset_builder()
        .with_data(&features_bytes)
        .create("id")?;
    features_group.new_dataset_builder()
        .with_data(&features_bytes)
        .create("name")?;

    Ok(())
}

fn sparse_matrix_to_csc(matrix: &Array2<f32>) -> (Vec<f32>, Vec<i32>, Vec<i32>) {
    let mut data = Vec::new();
    let mut indices = Vec::new();
    let mut indptr = vec![0];
    let mut nnz = 0;

    for col in matrix.axis_iter(Axis(1)) {
        for (row_idx, &value) in col.indexed_iter() {
            if value != 0.0 {
                data.push(value);
                indices.push(row_idx as i32);
                nnz += 1;
            }
        }
        indptr.push(nnz);
    }

    (data, indices, indptr)
}

fn mtx_2_h5(
    directory: &str,
    matrix_file: &str,
    feature_file: &str,
    barcode_file: &str,
    gene_column: usize,
    genome: &str,
    datatype: &str
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(directory).expect("Failed to create output directory");

    let filename = if datatype == "Peak" {
        format!("{}/peak_count_matrix.h5", directory)
    } else {
        format!("{}/gene_count_matrix.h5", directory)
    };

    let (matrix, features, barcodes) = read_10x_mtx(matrix_file, feature_file, barcode_file, datatype, gene_column)?;

    write_10x_h5(&filename, matrix, features, barcodes, genome, datatype)?;
    Ok(())
}

fn main() {
    let args = Args::parse();

    let matrix_file = format!("{}/matrix.mtx.gz", args.matrix);
    let feature_file = if args.datatype == "Peak" {
        format!("{}/peaks.bed.gz", args.matrix)
    } else {
        format!("{}/features.tsv.gz", args.matrix)
    };
    let barcode_file = format!("{}/barcodes.tsv.gz", args.matrix);

    mtx_2_h5(
        &args.outdir,
        &matrix_file,
        &feature_file,
        &barcode_file,
        args.column,
        &args.genome,
        &args.datatype,
    ).unwrap_or_else(|err| {
        eprintln!("Error processing data: {}", err);
        std::process::exit(1);
    });
}
