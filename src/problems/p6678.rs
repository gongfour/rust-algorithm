use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n_antenna = tokens.next().unwrap().parse::<usize>().unwrap();
    let n_eyes = tokens.next().unwrap().parse::<usize>().unwrap();

    if n_antenna >= 3 && n_eyes <= 4 {
        println!("TroyMartian");
    }
    if n_antenna <= 6 && n_eyes >= 2 {
        println!("VladSaturnian");
    }
    if n_antenna <= 2 && n_eyes <= 3 {
        println!("GraemeMercurian");
    }
}
