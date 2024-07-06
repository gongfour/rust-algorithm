pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for i in 1..=n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        print!("{i}. {input}");
    }
}
