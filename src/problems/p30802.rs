fn solve(n: usize, s: Vec<usize>, t: usize, p: usize) {
    let mut count = 0;
    for i in s {
        let s_a = i / t;
        let s_b = i % t;
        if s_b > 0 {
            count += 1;
        }
        count += s_a;
    }

    let p_a = n / p;
    let p_b = n % p;

    println!("{}", count);
    println!("{} {}", p_a, p_b);
}

pub fn main() {
    // read n
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // read s
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let mut s = Vec::new();
    tokens.for_each(|x| {
        let x: usize = x.parse().unwrap();
        s.push(x);
    });

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let t: usize = tokens.next().unwrap().parse().unwrap();
    let p: usize = tokens.next().unwrap().parse().unwrap();

    solve(n, s, t, p);
}
