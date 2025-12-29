use std::env::args;

struct Console {
    first_num: f32,
    second_num: f32,
    operator: char,
}

fn main() {
    let console: Console = Console::new(args().collect());
    console.calculator();
}

impl Console {
    fn new(arg: Vec<String>) -> Console {
        Console {
            first_num: arg[1].parse().unwrap(),
            second_num: arg[3].parse().unwrap(),
            operator: arg[2].parse().unwrap(),
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
