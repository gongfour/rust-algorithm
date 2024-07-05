use std::io::Read;

fn youngsik(a: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += (a[i] / 30) * 10 + 10;
    }
    sum
}

fn minsik(a: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..a.len() {
        sum += (a[i] / 60) * 15 + 15;
    }
    sum
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        a.push(tokens.next().unwrap().parse().unwrap());
    }

    let youngsik = youngsik(&a);
    let minsik = minsik(&a);

    if youngsik < minsik {
        println!("Y {}", youngsik);
    } else if youngsik > minsik {
        println!("M {}", minsik);
    } else {
        println!("Y M {}", youngsik);
    }
}
