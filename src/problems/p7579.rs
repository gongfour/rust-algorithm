use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().flat_map(str::parse::<usize>);

    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();

    let b: Vec<usize> = tokens.by_ref().take(n).collect();
    let c: Vec<usize> = tokens.by_ref().take(n).collect();
    let sum = c.iter().sum::<usize>();

    let mut d = vec![0; sum + 1];

    for i in 1..=n {
        for j in (0..=sum).rev() {
            if j >= c[i - 1] {
                d[j] = d[j].max(d[j - c[i - 1]] + b[i - 1]);
            }
        }
    }

    println!("{:?}", d);

    let idx = d
        .iter()
        .enumerate()
        .find(|(_, &x)| x >= m)
        .map(|(i, _)| i)
        .unwrap();

    println!("{}", idx);
}
