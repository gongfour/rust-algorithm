pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut d = vec![0; n + 1];
    d[0] = 1;
    d[1] = 3;

    for i in 2..n {
        d[i] = (d[i - 1] + d[i - 2] * 2) % 10007;
    }
    println!("{}", d[n - 1]);
}
