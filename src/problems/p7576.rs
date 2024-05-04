use std::collections::VecDeque;
use std::io::Read;

fn bfs(
    n: usize,
    m: usize,
    boxs: &mut Vec<Vec<i32>>,
    queue: &mut VecDeque<(usize, usize, i32)>,
    visited: &mut Vec<Vec<i32>>,
) {
    while let Some((y, x, d)) = queue.pop_front() {
        if visited[y][x] != -1 {
            continue;
        }
        visited[y][x] = d;

        if y > 0 && boxs[y - 1][x] == 0 {
            queue.push_back((y - 1, x, d + 1));
        }
        if y < n - 1 && boxs[y + 1][x] == 0 {
            queue.push_back((y + 1, x, d + 1));
        }
        if x > 0 && boxs[y][x - 1] == 0 {
            queue.push_back((y, x - 1, d + 1));
        }
        if x < m - 1 && boxs[y][x + 1] == 0 {
            queue.push_back((y, x + 1, d + 1));
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let m = iter.next().unwrap().parse::<usize>().unwrap();
    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let mut boxs = vec![vec![0; m]; n];
    let mut queue = VecDeque::new();
    for y in 0..n {
        for x in 0..m {
            boxs[y][x] = iter.next().unwrap().parse::<i32>().unwrap();
            if boxs[y][x] == 1 {
                queue.push_back((y, x, 0));
            }
        }
    }

    let mut visited = vec![vec![-1; m]; n];

    bfs(n, m, &mut boxs, &mut queue, &mut visited);

    let ans = visited.iter().flatten().max().unwrap();
    for y in 0..n {
        for x in 0..m {
            if visited[y][x] == -1 && boxs[y][x] == 0 {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", ans);
}
