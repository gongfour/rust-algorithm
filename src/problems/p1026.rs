use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    for i in 0..n {
        a[i] = tokens.next().unwrap().parse().unwrap();
    }
    for i in 0..n {
        b[i] = tokens.next().unwrap().parse().unwrap();
    }

    a.sort();
    b.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..n {
        sum += a[i] * b[i];
    }

    println!("{}", sum);
}
