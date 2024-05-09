use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().flat_map(str::parse::<i32>);

    loop {
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        if a == 0 && b == 0 {
            break;
        }
        if a > b {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
