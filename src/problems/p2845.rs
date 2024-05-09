pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let s = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<usize>)
        .reduce(|a, b| a * b)
        .unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let d = input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    for i in d {
        print!("{} ", i - s as i64);
    }
}
