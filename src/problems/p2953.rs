fn main() {
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..5 {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let sum: usize = s
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .sum();

        if sum > max {
            max = sum;
            max_index = i + 1;
        }
    }
    println!("{} {}", max_index, max);
}
