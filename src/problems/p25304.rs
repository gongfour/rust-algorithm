use std::io::Read;

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    let mut tokens = s.split_whitespace();
    let x: usize = tokens.next().unwrap().parse().unwrap();
    let n: usize = tokens.next().unwrap().parse().unwrap();

    let mut sum = 0;
    for _ in 0..n {
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let b: usize = tokens.next().unwrap().parse().unwrap();

        sum += a * b;
    }

    if x == sum {
        println!("Yes");
    } else {
        println!("No");
    }
}
