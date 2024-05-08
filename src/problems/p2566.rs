use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let v = input
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .collect::<Vec<usize>>();

    let (index, sum) = v
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(i, s)| (i, s))
        .unwrap();

    let row = index / 9;
    let col = index % 9;

    println!("{}", sum);
    println!("{} {}", row + 1, col + 1);
}
