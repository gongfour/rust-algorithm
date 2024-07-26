use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let mut max_people = 0;
    let mut current_people = 0;
    for _ in 0..10 {
        let output: usize = tokens.next().unwrap().parse().unwrap();
        let input: usize = tokens.next().unwrap().parse().unwrap();
        current_people -= output;
        current_people += input;
        if current_people > max_people {
            max_people = current_people;
        }
    }
    println!("{}", max_people);
}
