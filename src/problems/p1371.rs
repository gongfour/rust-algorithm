use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let chars = input.chars();

    let mut alphabets = vec![0; 26];
    for c in chars {
        if c.is_alphabetic() {
            let index = (c as u8 - b'a') as usize;
            alphabets[index] += 1;
        }
    }

    let mut index = 0;
    for i in 1..26 {
        if alphabets[i] > alphabets[index] {
            index = i;
        }
    }
    let value = alphabets[index];

    let answers: Vec<(usize, &usize)> = alphabets
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c == value)
        .collect();

    for i in answers {
        let c = (i.0 as u8 + b'a') as char;
        print!("{}", c);
    }
}
