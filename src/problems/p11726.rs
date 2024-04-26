pub fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = (dp[i - 1] + dp[i - 2]) % 10007;
    }

    println!("{}", dp[n]);
}
