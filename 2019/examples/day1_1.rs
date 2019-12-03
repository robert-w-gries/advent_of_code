use advent_of_code_2019::*;

#[derive(Debug)]
struct Module(u32);

impl Module {
    fn required_fuel(&self) -> u32 {
        self.0 / 3 - 2
    }
}

fn main() {
    let mut v = Vec::new();
    apply_to_lines("input/day1_1", |line| {
        match line {
            s if !s.is_empty() => v.push(Module(s.parse::<u32>().unwrap())),
            _ => (),
        }
    });
    let mut sum = 0;
    for m in v {
        sum += m.required_fuel();
    }
    println!("Sum = {}", sum);
}
