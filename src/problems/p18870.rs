use std::collections::HashMap;
use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let d = iter.map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let mut ed = d.clone();
    ed.sort();

    let mut h = HashMap::new();
    for (i, x) in ed.iter().enumerate() {
        if h.get(x).is_none() {
            h.insert(x, i);
        }
    }

    for x in d {
        print!("{} ", h[&x]);
    }
}
