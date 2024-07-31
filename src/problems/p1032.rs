use std::io::Read;

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let mut words = Vec::new();
    for _ in 0..n {
        let word = tokens.next().unwrap();
        words.push(word);
    }

    let len = words[0].len();
    'a: for i in 0..len {
        let c = words[0].chars().nth(i).unwrap();
        for j in 1..n {
            if c != words[j].chars().nth(i).unwrap() {
                print!("?");
                continue 'a;
            }
        }
        print!("{}", c);
    }
}
