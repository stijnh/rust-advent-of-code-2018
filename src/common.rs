use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_export]
macro_rules! iff {
    ($a:expr, $b:expr, $c:expr) => {
        if $a {
            $b
        } else {
            $c
        }
    };
}

pub fn read_file_lines(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
}
