use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    println!("{:?}", tokens);
    let v_number: usize = tokens.next().unwrap().parse().unwrap();
    let e_number: usize = tokens.next().unwrap().parse().unwrap();

    let buffer = tokens.map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
    let graph: Vec<&[u32]> = buffer.chunks(v_number).collect();
    println!("{:?}", graph);
}
