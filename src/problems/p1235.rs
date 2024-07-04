use std::io::Read;

fn check_dup(s: &Vec<&str>, n: usize) -> bool {
    for i in 0..s.len() {
        for j in i..s.len() {
            if i == j {
                continue;
            }
            let s_i = s[i].chars().rev().take(n);
            let s_j = s[j].chars().rev().take(n);

            if s_i.eq(s_j) {
                return false;
            }
        }
    }
    true
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut students = vec![];

    for _ in 0..n {
        students.push(tokens.next().unwrap());
    }

    for i in 0..students[0].len() {
        let b = check_dup(&students, i + 1);
        if b {
            println!("{}", i + 1);
            return;
        }
    }
}
