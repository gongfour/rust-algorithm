pub fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut tokens = s.split_ascii_whitespace();
        let mut scores = Vec::new();
        for _ in 0..6 {
            let score: u32 = tokens.next().unwrap().parse().unwrap();
            scores.push(score);
        }
        if scores.iter().all(|&x| x == 0) {
            break;
        }
        scores.sort_unstable();
        let mut sum = 0;
        for i in 1..5 {
            sum += scores[i];
        }
        let answer: f64 = sum as f64 / 4.0;
        println!("{}", answer);
    }
}
