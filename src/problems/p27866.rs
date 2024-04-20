use std::io::{self};

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let string: String = input.parse().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let idx: usize = input.parse().unwrap();

    println!("{}", string.chars().nth(idx - 1).unwrap());
}
