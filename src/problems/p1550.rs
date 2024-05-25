fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim();
    let n = i64::from_str_radix(n, 16).unwrap();

    println!("{}", n);
}
