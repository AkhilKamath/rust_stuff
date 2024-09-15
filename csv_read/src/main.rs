use std::{error::Error, process, path::Path};
use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  input: String
}

fn main() {
    if let Err(err) = run() {
      println!("{}", err);
      process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();
  // let file_path = args.input;
  let file_path = Path::new(&args.input);
  // let file = File::open(file_path)?;
  // let mut rdr = csv::Reader::from_reader(file);

  let mut rdr = csv::Reader::from_path(file_path)?;

  for result in rdr.records() {
    let record = result?;
    println!("{:?}", record);
  }

  Ok(())
}