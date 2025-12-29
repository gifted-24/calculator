use std::env::args;

struct Console {
    first_num: f32,
    second_num: f32,
    operator: char,
    result: f32,
    error: String,
}

fn main() {
    let mut console: Console = Console::new(args().collect());
    console.calculator();
    console.output();
}

impl Console {
    fn new(arg: Vec<String>) -> Console {
        Console {
            first_num: arg[1].parse().unwrap(),
            second_num: arg[3].parse().unwrap(),
            operator: arg[2].parse().unwrap(),
            result: 0.0,
            error: String::new(),
        }
    }

    fn calculator(&mut self) {
        match self.operator {
            '+' => {
                self.result = self.first_num + self.second_num;
            }
            '-' => {
                self.result = self.first_num - self.second_num;
            }
            '*' | 'x' | 'X' => {
                self.result = self.first_num * self.second_num;
            }
            '/' => {
                self.result = self.first_num / self.second_num;
            }
            _ => {
                self.error = format!(
                    "Invalid Operation: [{} {} {}]",
                    self.first_num, self.operator, self.second_num
                );
            }
        }
    }

    fn output(&self) {
        match self.result {
            0.0 => println!("{}", self.error),
            _ => println!(
                "{} {} {} = {}",
                self.first_num, self.operator, self.second_num, self.result
            ),
        }
    }
}
