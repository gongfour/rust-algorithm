pub fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut count = 0;
        for c in input.chars() {
            match c {
                'a' => count += 1,
                'A' => count += 1,
                'e' => count += 1,
                'E' => count += 1,
                'i' => count += 1,
                'I' => count += 1,
                'o' => count += 1,
                'O' => count += 1,
                'u' => count += 1,
                'U' => count += 1,
                '#' => return,
                _ => (),
            }
        }
        println!("{}", count);
    }
}
