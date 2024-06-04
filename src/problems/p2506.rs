use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let mut count = 0;
    let mut sum = 0;
    for _ in 0..n {
        let a: i32 = tokens.next().unwrap().parse().unwrap();
        if a == 1 {
            sum += a;
            sum += count;
            count += 1;
        } else {
            count = 0;
        }
    }
    println!("{}", sum);
}
