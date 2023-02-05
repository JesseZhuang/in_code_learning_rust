use std::{fs, io};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;

/// combines csv files with common header in current directory
fn main() -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir("./")?;
    let mut csv_file_counter = 0;
    for path in paths {
        let path = path?.path();
        let mut out_file = BufWriter::new(
            OpenOptions::new().append(true).create(true).open("combined_output.csv")?);
        if let Some("csv") = path.extension().and_then(OsStr::to_str) {
            csv_file_counter += 1;
            let file_name = path.file_name().and_then(OsStr::to_str).unwrap();
            println!("processing csv file: {:?}", file_name);
            let skip = if csv_file_counter == 1 {0} else {1};
            for line in read_lines(file_name)?.skip(skip) {
                let line = line?;
                writeln!(out_file, "{}", line)?;
            }
        }
    }
    if csv_file_counter > 0 {
        println!("generated combined_output.csv.");
    }
    return Ok(());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
