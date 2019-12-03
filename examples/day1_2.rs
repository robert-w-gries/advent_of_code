use advent_of_code_2019::*;

#[derive(Debug)]
struct Module(i64);

impl Module {
    fn required_fuel(&self) -> i64 {
        let fuel = self.0 / 3 - 2;
        match fuel {
            x if x > 0 => {
                x + Module(x).required_fuel()
            },
            _ => 0,
        }
    }
}

fn main() {
    let mut v = Vec::new();
    apply_to_lines("input/day1_2", |line| {
        match line {
            s if !s.is_empty() => v.push(Module(s.parse::<i64>().unwrap())),
            _ => (),
        }
    });
    let mut sum = 0;
    for m in v {
        sum += m.required_fuel();
    }
    println!("Sum = {}", sum);
}
