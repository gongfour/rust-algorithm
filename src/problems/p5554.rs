use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let a: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();
    let d: usize = tokens.next().unwrap().parse().unwrap();

    let seconds = a + b + c + d;
    println!("{}", seconds / 60);
    println!("{}", seconds % 60);
}
