use std::io::Read;

fn print_fizzbuzz(n: usize) {
    if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", n);
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let a = tokens.next().unwrap();
    let b = tokens.next().unwrap();
    let c = tokens.next().unwrap();

    if let Ok(ra) = a.parse::<usize>() {
        let d = ra + 3;
        print_fizzbuzz(d);
        return;
    } else if let Ok(rb) = b.parse::<usize>() {
        let d = rb + 2;
        print_fizzbuzz(d);
    } else if let Ok(rc) = c.parse::<usize>() {
        let d = rc + 1;
        print_fizzbuzz(d);
        return;
    } else {
        println!("Error");
        return;
    }
}
