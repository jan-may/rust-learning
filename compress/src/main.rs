extern crate flate2;

use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Result, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: compress <input> <output>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let input = File::open(input_file)?;
    let output = File::create(output_file)?;

    let mut encoder =
        flate2::write::GzEncoder::new(BufWriter::new(output), flate2::Compression::default());
    let start = Instant::now();
    io::copy(&mut BufReader::new(input), &mut encoder)?;
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}
