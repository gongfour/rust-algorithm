use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut words = Vec::new();
    for _ in 0..n {
        words.push(tokens.next().unwrap());
    }

    words.sort();
    let mut is_prefix = Vec::new();
    for i in 0..n {
        is_prefix.push(false);
        for j in i + 1..n {
            if words[j].starts_with(words[i]) {
                is_prefix[i] = true;
                break;
            }
        }
    }

    let answer = is_prefix.iter().filter(|&&x| !x).count();
    println!("{}", answer);
}
