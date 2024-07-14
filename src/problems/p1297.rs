use std::io::Read;
pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let d: f64 = tokens.next().unwrap().parse().unwrap();
    let h: f64 = tokens.next().unwrap().parse().unwrap();
    let w: f64 = tokens.next().unwrap().parse().unwrap();

    let a = d * d / (w * w + h * h);
    let a = a.sqrt();
    println!("{:.0} {:.0}", (a * h).floor(), (a * w).floor());
}
