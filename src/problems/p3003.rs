use std::fmt::Write;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let r = [1, 1, 2, 2, 2, 8];
    let mut v = s
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut o = String::new();
    let d: Vec<i32> = v.iter().zip(r.iter()).map(|(x, y)| y - x).collect();
    for i in d.iter() {
        write!(&mut o, "{} ", i).unwrap();
    }
    println!("{}", o);
}
