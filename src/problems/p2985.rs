pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let a = tokens.next().unwrap().parse::<i32>().unwrap();
    let b = tokens.next().unwrap().parse::<i32>().unwrap();
    let c = tokens.next().unwrap().parse::<i32>().unwrap();

    if a + b == c {
        println!("{}+{}={}", a, b, c);
    } else if a - b == c {
        println!("{}-{}={}", a, b, c);
    } else if a * b == c {
        println!("{}*{}={}", a, b, c);
    } else if a / b == c && a % b == 0 {
        println!("{}/{}={}", a, b, c);
    } else if a == b + c {
        println!("{}={}+{}", a, b, c);
    } else if a == b - c {
        println!("{}={}-{}", a, b, c);
    } else if a == b * c {
        println!("{}={}*{}", a, b, c);
    } else if a == b / c && b % c == 0 {
        println!("{}={}/{}", a, b, c);
    } else if a == b && b == c {
        println!("{}={}={}", a, b, c);
    }
    return;
}
