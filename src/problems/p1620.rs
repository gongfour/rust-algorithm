use std::{
    collections::HashMap,
    fmt::Write,
    io::{stdin, Read},
};

pub fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<String>);

    let n = input.next().unwrap().parse::<usize>().unwrap();
    let m = input.next().unwrap().parse::<usize>().unwrap();

    let mut idx_map: HashMap<usize, String> = HashMap::new();
    let mut str_map: HashMap<String, usize> = HashMap::new();
    input.clone().take(n).for_each(|p| {
        idx_map.insert(idx_map.len() + 1, p.clone());
        str_map.insert(p, str_map.len() + 1);
    });

    let mut output = String::new();
    input.skip(n).take(m).for_each(|t| {
        if let Ok(idx) = t.parse::<usize>() {
            write!(output, "{}\n", idx_map.get(&idx).unwrap());
        } else {
            write!(output, "{}\n", str_map.get(&t).unwrap());
        }
    });
    println!("{}", output);
}
