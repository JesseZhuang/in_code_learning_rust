use clap::{App, Arg};
use std::io;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{BufRead, BufWriter, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_FILE: &str = "inputFile";
    const CONSTANT_VALUE: &str = "constantValue";
    let matches = App::new("Generate comma separated list")
        .version("1.0")
        .author("Jesse Zhuang")
        .about("Generate 2 comma separated list with constant value in second list.")
        .arg(Arg::with_name(INPUT_FILE)
            .short("i")
            .long(INPUT_FILE)
            .value_name(INPUT_FILE)
            .help("input file with multiple rows, the rows with be joined with comma")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name(CONSTANT_VALUE)
            .short("c")
            .long(CONSTANT_VALUE)
            .value_name(CONSTANT_VALUE)
            .help("this constant value will be joined with comma. number of times \
             repeating is determined by number of rows in inputFile.")
            .takes_value(true)
            .required(true))
        .get_matches();

    let input_file = matches.value_of(INPUT_FILE).expect("inputFile was required");
    println!("Input file is {}.", input_file);
    let constant_value = matches.value_of(CONSTANT_VALUE).expect("constant value required");
    println!("Constant value is {}", constant_value);

    let mut out_file = BufWriter::new(
        OpenOptions::new().append(true).create(true).open("output.txt")?);
    let mut s1 = Vec::new();
    let mut s2 = Vec::new();
    for line in read_lines(input_file)? {
        s1.push(line?);
        s2.push(constant_value);
    }

    const COMMA: &str = ",";
    println!("writing to output file {}.", out_file);
    writeln!(out_file, "{}", s1.join(COMMA))?;
    writeln!(out_file, "{}", s2.join(COMMA))?;
    return Ok(());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}