extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
// use std::env::args;
use std::fs::File;
use std::io;
use std::io::copy;
use std::io::{ BufReader, BufWriter };
use std::time::Instant;
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let input_path = Path::new(&args.input);
    let output_path = Path::new(&args.output);

    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;

    let mut input_buf_reader = BufReader::new(input_file);
    let output_buf_writer = BufWriter::new(output_file);

    let mut encoder = GzEncoder::new(output_buf_writer, Compression::default());

    // let input_file = Path::new()
    // let f = File::open(args().nth(1).unwrap())?;
    // let mut input = BufReader::new(f);
    // let output = File::create(args().nth(2).unwrap()).unwrap();
    // let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input_buf_reader, &mut encoder)?;
    let _output = encoder.finish()?;
    let duration = start.elapsed();

    let input_len = input_path.metadata()?.len();
    let output_len = output_path.metadata()?.len();
    
    println!(
        "Source length: {:?}",
        input_len
    );

    println!(
        "Target length: {:?}",
        output_len
    );

    println!(
        "Compression ratio: {:.2}%",
        (output_len as f64 / input_len as f64) * 100.0
    );


    println!(
        "Elapsed time: {:?}",
        duration
    );
    Ok(())
}