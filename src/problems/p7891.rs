use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let t = tokens.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let a = tokens.next().unwrap().parse::<i64>().unwrap();
        let b = tokens.next().unwrap().parse::<i64>().unwrap();
        println!("{}", a + b);
    }
}
