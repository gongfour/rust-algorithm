use std::fmt::Write;

pub fn main() {
    let mut answer = String::new();
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut tokens = s.split_whitespace();
        let numerator: usize = tokens.next().unwrap().parse().unwrap();
        let denominator: usize = tokens.next().unwrap().parse().unwrap();

        if numerator == 0 && denominator == 0 {
            break;
        }

        let quotient = numerator / denominator;
        let remainder = numerator % denominator;
        writeln!(answer, "{} {} / {}", quotient, remainder, denominator).unwrap();
    }
    print!("{}", answer);
}
