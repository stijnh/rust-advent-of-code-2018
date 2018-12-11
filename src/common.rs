use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
}
