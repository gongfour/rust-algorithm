use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let a = tokens.next().unwrap().chars().collect::<Vec<_>>();

    let b = tokens.next().unwrap().chars().collect::<Vec<_>>();

    let sum: u128 = a
        .iter()
        .flat_map(|&x| {
            b.iter()
                .map(move |&y| x.to_digit(10).unwrap() * y.to_digit(10).unwrap())
        })
        .fold(0 as u128, |acc, val| acc + val as u128);

    println!("{}", sum);
}
