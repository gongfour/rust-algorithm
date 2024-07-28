use std::io::Read;

fn count_numbers(s: &str) -> usize {
    let mut sum = 0;
    for c in s.chars() {
        match c {
            '0' => sum += 0,
            '1' => sum += 1,
            '2' => sum += 2,
            '3' => sum += 3,
            '4' => sum += 4,
            '5' => sum += 5,
            '6' => sum += 6,
            '7' => sum += 7,
            '8' => sum += 8,
            '9' => sum += 9,
            _ => {}
        }
    }
    sum
}

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut serials = Vec::new();
    for _ in 0..n {
        serials.push(tokens.next().unwrap());
    }

    serials.sort_by(|a, b| {
        if a.len() != b.len() {
            a.len().cmp(&b.len())
        } else {
            let sum_a = count_numbers(a);
            let sum_b = count_numbers(b);
            if sum_a != sum_b {
                sum_a.cmp(&sum_b)
            } else {
                a.cmp(b)
            }
        }
    });

    println!("{}", serials.join("\n"));
}
