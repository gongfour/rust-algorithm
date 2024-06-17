use std::collections::HashMap;
use std::collections::VecDeque;

fn swap(s: &mut Vec<char>, i: usize, j: usize) {
    let tmp = s[i];
    s[i] = s[j];
    s[j] = tmp;
}

fn bfs(n: &Vec<char>, k: usize, ans: &mut usize) -> bool {
    let mut is_reached = false;
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((n.clone(), 0));

    while let Some((v, idx)) = q.pop_front() {
        if let Some(_) = visited.get(&(idx, v.clone())) {
            continue;
        }
        visited.insert((idx, v.clone()), true);

        if k == idx {
            is_reached = true;
            let s: String = v.into_iter().collect();
            let u: usize = s.parse().unwrap();

            if u > *ans {
                *ans = u;
            }
            continue;
        }

        for i in 0..n.len() {
            for j in i + 1..n.len() {
                let mut c = v.clone();
                swap(&mut c, i, j);
                if c.first() == Some(&'0') {
                    continue;
                }
                q.push_back((c, idx + 1));
            }
        }
    }
    is_reached
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let n: Vec<char> = tokens.next().unwrap().chars().collect();
    let k: usize = tokens.next().unwrap().parse().unwrap();

    let mut answer = 0;
    let is_reached = bfs(&n, k, &mut answer);
    match is_reached {
        true => println!("{}", answer),
        false => println!("-1"),
    }
}
