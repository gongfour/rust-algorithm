pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut tokens = s.split_whitespace();
    let a: i32 = tokens.next().unwrap().parse().unwrap();
    let b: i32 = tokens.next().unwrap().parse().unwrap();

    println!("{}", a ^ b);
}
