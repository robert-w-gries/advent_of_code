use std::fs::File;
use std::io::{prelude::*, BufReader};

// answer = 3085697
fn main() {
    let mut v: Vec<u64> = {
        let file = File::open("input/day2_1").expect("Could not open file");
        let mut reader = BufReader::new(file);

        let mut buf = String::new();
        reader.read_line(&mut buf).expect("Could not read line");
        buf.trim().split(',').map(|s| s.parse().unwrap()).collect()
    };
    for i in (0..v.len()).step_by(4) {
        match v[i] {
            1 => {
                let (add1, add2, store) = (v[i+1], v[i+2], v[i+3]);
                v[store as usize] = v[add1 as usize] + v[add2 as usize];
            },
            2 => {
                let (mul1, mul2, store) = (v[i+1], v[i+2], v[i+3]);
                v[store as usize] = v[mul1 as usize] * v[mul2 as usize];
            },
            99 => break,
            _ => panic!("Invalid input"),
        }
    }
    println!("v[0] = {:?}", v[0]);
}
