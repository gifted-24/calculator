use std::env::{args, Args};

struct Console {
    first_num: f32,
    second_num: f32,
    operator: char,
}

fn main() {
    let console: Console = Console::new();
    console.calculator();
}

impl Console {
    fn new() -> Console {
        let mut args: Args = args();
        Console {
            first_num: args.nth(1).unwrap().parse::<f32>().unwrap(),
            operator: args.next().unwrap().parse::<char>().unwrap(),
            second_num: args.next().unwrap().parse::<f32>().unwrap(),
        }
    }

    fn calculator(&self) {
        match self.operator {
            '+' => println!(
                "{} + {} = {}",
                self.first_num,
                self.second_num,
                self.first_num + self.second_num
            ),
            '-' => println!(
                "{} - {} = {}",
                self.first_num,
                self.second_num,
                self.first_num - self.second_num
            ),
            '*' | 'x' | 'X' => println!(
                "{} * {} = {}",
                self.first_num,
                self.second_num,
                self.first_num * self.second_num
            ),
            '/' => println!(
                "{} / {} = {}",
                self.first_num,
                self.second_num,
                self.first_num / self.second_num
            ),
            _ => println!(
                "Invalid Operation: [{} {} {}]",
                self.first_num, self.operator, self.second_num
            ),
        }
    }
}
