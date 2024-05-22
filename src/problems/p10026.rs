use std::collections::VecDeque;
use std::io::Read;

fn bfs(m: &Vec<Vec<char>>, s: (usize, usize), v: &mut Vec<Vec<bool>>, rg: bool) {
    let n = m.len() as isize;
    let c = m[s.0][s.1];
    let mut q = VecDeque::new();
    q.push_back(s);

    while let Some((y, x)) = q.pop_front() {
        if v[y][x] {
            continue;
        }
        v[y][x] = true;
        let y = y as isize;
        let x = x as isize;
        let adjs = [
            ((y - 1).max(0), x),
            ((y + 1).min(n - 1), x),
            (y, (x - 1).max(0)),
            (y, (x + 1).min(n - 1)),
        ];

        for (ny, nx) in adjs.iter() {
            if c == m[*ny as usize][*nx as usize] {
                q.push_back((*ny as usize, *nx as usize));
            }

            if rg && c == 'R' && m[*ny as usize][*nx as usize] == 'G' {
                q.push_back((*ny as usize, *nx as usize));
            }
            if rg && c == 'G' && m[*ny as usize][*nx as usize] == 'R' {
                q.push_back((*ny as usize, *nx as usize));
            }
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut m = vec![vec!['R'; n]; n];
    for i in 0..n {
        let line = iter.next().unwrap();
        for (j, c) in line.chars().enumerate() {
            m[i][j] = c;
        }
    }

    let mut visited = vec![vec![false; n]; n];
    let mut visited_rg = vec![vec![false; n]; n];

    let mut count = 0;
    let mut count_rg = 0;
    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                bfs(&m, (i, j), &mut visited, false);
                count += 1;
            }
            if !visited_rg[i][j] {
                bfs(&m, (i, j), &mut visited_rg, true);
                count_rg += 1;
            }
        }
    }

    println!("{} {}", count, count_rg)
}