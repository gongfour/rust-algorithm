fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();

        let name = tokens.next().unwrap();
        let age: usize = tokens.next().unwrap().parse().unwrap();
        let weight: usize = tokens.next().unwrap().parse().unwrap();

        if name == "#" && age == 0 && weight == 0 {
            break;
        }

        if age > 17 || weight >= 80 {
            println!("{} Senior", name);
        } else {
            println!("{} Junior", name);
        }
    }
}
