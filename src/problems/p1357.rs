pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut tokens = s.split_ascii_whitespace();

    let rev_a = tokens
        .next()
        .unwrap()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let rev_b = tokens
        .next()
        .unwrap()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let rev_ans = rev_a + rev_b;
    let ans = rev_ans
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    println!("{}", ans);
}
