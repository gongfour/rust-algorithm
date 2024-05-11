use std::fmt::Write;

fn backtracking(
    n: usize,
    m: usize,
    v: &Vec<usize>,
    selected: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    answer: &mut Vec<Vec<usize>>,
) {
    if selected.len() == m {
        answer.push(selected.clone());
        return;
    }

    let mut prev = 0;
    for i in 0..n {
        if visited[i] {
            continue;
        }
        if prev == v[i] {
            continue;
        }
        prev = v[i];

        visited[i] = true;
        selected.push(v[i]);
        backtracking(n, m, v, selected, visited, answer);
        selected.pop();
        visited[i] = false;
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let m = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut v = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    v.sort();

    let mut answers = vec![];
    backtracking(n, m, &v, &mut vec![], &mut vec![false; n], &mut answers);

    let mut answer = String::new();
    for s in answers {
        if s.is_empty() {
            continue;
        }
        for v in s {
            write!(answer, "{} ", v).unwrap();
        }
        writeln!(answer).unwrap();
    }
    println!("{}", answer);
}
