#![allow(dead_code, unused_variables)]

//! TODO

use regex::RegexSet;

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// TODO
fn read_file_line_by_line() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    let set = RegexSet::new(&[
        r"\w+",
        r"\d+",
        r"\pL+",
        r"foo",
        r"bar",
        r"barfoo",
        r"foobar",
    ]).unwrap();

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
