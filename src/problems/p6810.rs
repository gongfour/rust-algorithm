use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();
    let a: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();

    let sum = a + b * 3 + c;
    println!("The 1-3-sum is {}", sum + 91);
}
