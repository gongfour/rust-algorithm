use std::io::Read;

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_whitespace();

    let h: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let s: usize = tokens.next().unwrap().parse().unwrap();

    let t: usize = tokens.next().unwrap().parse().unwrap();

    let mut s = s + t;
    let mut m = m + s / 60;
    s %= 60;
    let mut h = h + m / 60;
    m %= 60;
    h %= 24;

    println!("{} {} {}", h, m, s);
}
