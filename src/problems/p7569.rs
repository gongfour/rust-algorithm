use std::collections::VecDeque;
use std::io::Read;

type Map = Vec<Vec<Vec<i32>>>;
type Pos = (usize, usize, usize);
type Visited = Vec<Vec<Vec<bool>>>;

fn bfs(map: &Map, s: &mut Vec<Pos>) -> (Visited, usize) {
    let mut visited = vec![vec![vec![false; map[0][0].len()]; map[0].len()]; map.len()];
    let mut q = VecDeque::new();
    for &start in s.iter() {
        q.push_back((start, 0));
    }

    let mut day = 0;

    while let Some(((i, j, k), d)) = q.pop_front() {
        if visited[i][j][k] {
            continue;
        }
        // println!("{}, {}, {}: {}", i, j, k, d);
        visited[i][j][k] = true;
        day = d;
        if i > 0 && map[i - 1][j][k] == 0 {
            q.push_back(((i - 1, j, k), d + 1));
        }
        if i < map.len() - 1 && map[i + 1][j][k] == 0 {
            q.push_back(((i + 1, j, k), d + 1));
        }
        if j > 0 && map[i][j - 1][k] == 0 {
            q.push_back(((i, j - 1, k), d + 1));
        }
        if j < map[0].len() - 1 && map[i][j + 1][k] == 0 {
            q.push_back(((i, j + 1, k), d + 1));
        }
        if k > 0 && map[i][j][k - 1] == 0 {
            q.push_back(((i, j, k - 1), d + 1));
        }
        if k < map[0][0].len() - 1 && map[i][j][k + 1] == 0 {
            q.push_back(((i, j, k + 1), d + 1));
        }
    }
    (visited, day)
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let m = tokens.next().unwrap().parse::<usize>().unwrap();
    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let h = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut map: Map = vec![vec![vec![0; m]; n]; h];
    let mut starts: Vec<Pos> = Vec::new();
    for i in 0..h {
        for j in 0..n {
            for k in 0..m {
                map[i][j][k] = tokens.next().unwrap().parse::<i32>().unwrap();
                if map[i][j][k] == 1 {
                    starts.push((i, j, k));
                }
            }
        }
    }

    let all = map
        .iter()
        .all(|x| x.iter().all(|y| y.iter().all(|z| *z == 1 || *z == -1)));
    if all {
        println!("0");
        return;
    }

    let (visited, day) = bfs(&map, &mut starts);

    for i in 0..h {
        for j in 0..n {
            for k in 0..m {
                if !visited[i][j][k] && map[i][j][k] == 0 {
                    println!("-1");
                    return;
                }
            }
        }
    }
    println!("{}", day);
}
