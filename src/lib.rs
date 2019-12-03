use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn apply_to_lines<F>(path: &str, mut f: F) where
    F: FnMut(String)
{
    let file = File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        f(line.expect("Could not read line"));
    }
}
