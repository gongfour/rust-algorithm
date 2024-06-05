use std::io::Read;

fn solve(
    p: &Vec<f64>,
    n: usize,
    pos: (isize, isize),
    count: f64,
    visited: &mut Vec<Vec<bool>>,
    path: &mut Vec<char>,
    prob: &mut f64,
) {
    let y = pos.0 as usize;
    let x = pos.1 as usize;

    if visited[y][x] {
        return;
    }
    visited[y][x] = true;

    if n == path.len() {
        visited[y][x] = false;
        *prob += count;
        return;
    }

    path.push('E');
    solve(p, n, (pos.0, pos.1 + 1), count * p[0], visited, path, prob);
    path.pop();
    path.push('W');
    solve(p, n, (pos.0, pos.1 - 1), count * p[1], visited, path, prob);
    path.pop();
    path.push('N');
    solve(p, n, (pos.0 + 1, pos.1), count * p[2], visited, path, prob);
    path.pop();
    path.push('S');
    solve(p, n, (pos.0 - 1, pos.1), count * p[3], visited, path, prob);
    path.pop();

    visited[y][x] = false;
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let p = tokens
        .flat_map(str::parse::<f64>)
        .map(|x| x * 0.01)
        .collect::<Vec<_>>();

    let mut prob = 0.0;
    let mut visited = vec![vec![false; n * 2 + 1]; n * 2 + 1];
    solve(
        &p,
        n,
        (n as isize, n as isize),
        1.0,
        &mut visited,
        &mut Vec::new(),
        &mut prob,
    );
    println!("{}", prob);
}
