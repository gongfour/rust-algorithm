fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut tokens = s.split_whitespace();
    let a: usize = tokens.next().unwrap().parse().unwrap();
    let b: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();

    if a + b + c >= 100 {
        println!("OK");
    } else if a < b && a < c {
        println!("Soongsil");
    } else if b < a && b < c {
        println!("Korea");
    } else {
        println!("Hanyang");
    }
}
