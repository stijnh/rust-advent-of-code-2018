use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_file_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    BufReader::new(f)
            .lines()
            .map(|l| l.unwrap())
            .collect::<Vec<_>>()
}
