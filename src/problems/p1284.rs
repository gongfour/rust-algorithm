pub fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let chars = input.trim().chars();

        if chars.clone().all(|c| c == '0') {
            break;
        }

        let mut sum = 1;
        for c in chars {
            sum += match c {
                '0' => 4,
                '1' => 2,
                _ => 3,
            };
            sum += 1;
        }
        println!("{}", sum);
    }
}
