use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let t: usize = tokens.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = tokens.next().unwrap().parse().unwrap();
        for _ in 0..n {
            print!("=");
        }
        println!();
    }
}
