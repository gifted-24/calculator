
use std::str::Chars;
fn main() {
    let string: String = String::from("Test");
    let char1 = string.chars();
    match char1 {
        Chars(val) => println!("{:#?}", val),
        _ => {},
    }
}