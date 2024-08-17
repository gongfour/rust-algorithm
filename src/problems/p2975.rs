pub fn main() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut tokens = line.split_whitespace();

        let account = tokens.next().unwrap().parse::<i32>().unwrap();

        let action = tokens.next().unwrap();

        let money = tokens.next().unwrap().parse::<i32>().unwrap();

        if account == 0 && action == "W" && money == 0 {
            break;
        }

        if action == "W" && account - money < -200 {
            println!("Not allowed");
        } else if action == "W" {
            println!("{}", account - money);
        } else {
            println!("{}", account + money);
        }
    }
}
