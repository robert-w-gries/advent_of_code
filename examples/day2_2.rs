use std::fs::File;
use std::io::{prelude::*, BufReader};

fn intcode_execute(mut memory: Vec<u32>, noun: u32, verb: u32) -> bool {
    memory[1] = noun;
    memory[2] = verb;
    for i in (0..memory.len()).step_by(4) {
        match memory[i] {
            1 => {
                let (add1, add2, store) = (memory[i+1], memory[i+2], memory[i+3]);
                memory[store as usize] = memory[add1 as usize] + memory[add2 as usize];
            },
            2 => {
                let (mul1, mul2, store) = (memory[i+1], memory[i+2], memory[i+3]);
                memory[store as usize] = memory[mul1 as usize] * memory[mul2 as usize];
            },
            99 => break,
            _ => panic!("Invalid input"),
        }
    }
    return memory[0] == 19690720;
}

fn main() {
    let memory: Vec<u32> = {
        // use same input from previous puzzle
        let file = File::open("input/day2_1").expect("Could not open file");
        let mut reader = BufReader::new(file);

        let mut buf = String::new();
        reader.read_line(&mut buf).expect("Could not read line");
        buf.trim().split(',').map(|s| s.parse().unwrap()).collect()
    };
    for noun in 0..100 {
        for verb in 0..100 {
            if intcode_execute(memory.clone(), noun, verb) {
                println!("Answer = {}", 100 * noun + verb);
            }
        }
    }
}
