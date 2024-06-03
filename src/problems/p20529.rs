use std::collections::HashMap;
use std::fmt::Write;

fn distance2(p1: &str, p2: &str) -> usize {
    let p1 = p1.chars().collect::<Vec<char>>();
    let p2 = p2.chars().collect::<Vec<char>>();

    let mut dist = 0;
    for (c1, c2) in p1.iter().zip(p2.iter()) {
        if c1 != c2 {
            dist += 1;
        }
    }
    dist
}

fn distance3(p1: &str, p2: &str, p3: &str) -> usize {
    distance2(p1, p2) + distance2(p2, p3) + distance2(p3, p1)
}

fn solve(ps: Vec<&str>) -> usize {
    let n = ps.len();
    let mut min_dist = std::usize::MAX;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                let dist = distance3(ps[i], ps[j], ps[k]);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
    }
    min_dist
}

fn check_three(ps: &Vec<&str>) -> bool {
    let mut hash: HashMap<&str, i32> = HashMap::new();
    for &p in ps {
        match hash.get(p) {
            Some(c) => {
                hash.insert(p, c + 1);
            }
            None => {
                hash.insert(p, 1);
            }
        }
    }

    hash.iter().any(|(_, &c)| c >= 3)
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();
    let mut output = String::new();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let n = input.trim().parse::<i32>().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let ps = input.split_whitespace().collect::<Vec<&str>>();

        if check_three(&ps) {
            writeln!(output, "0").unwrap();
            continue;
        }

        let answer = solve(ps);
        writeln!(output, "{}", answer).unwrap();
    }
    println!("{}", output);
}
