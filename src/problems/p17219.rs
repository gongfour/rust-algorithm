use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_ascii_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let m = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut hash = HashMap::new();
    for _ in 0..n {
        let key = tokens.next().unwrap().to_string();
        let value = tokens.next().unwrap().to_string();
        hash.insert(key, value);
    }

    let mut o = String::new();
    for _ in 0..m {
        let key = tokens.next().unwrap().to_string();
        writeln!(o, "{}", hash.get(&key).unwrap()).unwrap();
    }

    print!("{}", o);
}
