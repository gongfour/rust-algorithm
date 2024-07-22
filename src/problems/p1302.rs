use std::collections::HashMap;
use std::io::Read;
pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut map = HashMap::new();
    for _ in 0..n {
        let word = tokens.next().unwrap();
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    map.iter()
        .max_by(|a, b| {
            let value_order = a.1.cmp(b.1);
            if value_order == std::cmp::Ordering::Equal {
                b.0.cmp(a.0)
            } else {
                value_order
            }
        })
        .map(|(word, _)| {
            println!("{}", word);
        });
}
