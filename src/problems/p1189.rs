use std::io::Read;

type Map = Vec<Vec<char>>;
type Pos = (usize, usize);

fn dfs(
    map: &Map,
    current: Pos,
    end: Pos,
    k: usize,
    count: usize,
    visited: &mut Vec<Vec<bool>>,
    answer: &mut usize,
) {
    if visited[current.0][current.1] {
        return;
    }
    if map[current.0][current.1] == 'T' {
        return;
    }
    if current == end {
        if k == count {
            *answer += 1;
        }
        return;
    }
    visited[current.0][current.1] = true;

    if current.0 > 0 {
        dfs(
            map,
            (current.0 - 1, current.1),
            end,
            k,
            count + 1,
            visited,
            answer,
        );
    }
    if current.0 < map.len() - 1 {
        dfs(
            map,
            (current.0 + 1, current.1),
            end,
            k,
            count + 1,
            visited,
            answer,
        );
    }
    if current.1 > 0 {
        dfs(
            map,
            (current.0, current.1 - 1),
            end,
            k,
            count + 1,
            visited,
            answer,
        );
    }
    if current.1 < map[0].len() - 1 {
        dfs(
            map,
            (current.0, current.1 + 1),
            end,
            k,
            count + 1,
            visited,
            answer,
        );
    }

    visited[current.0][current.1] = false;
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let r: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();

    let mut map = vec![vec!['.'; c]; r];
    for y in 0..r {
        for (x, ch) in tokens.next().unwrap().chars().enumerate() {
            map[y][x] = ch;
        }
    }

    let mut answer = 0;
    let mut visited = vec![vec![false; c]; r];
    dfs(
        &map,
        (r - 1, 0),
        (0, c - 1),
        k,
        1,
        &mut visited,
        &mut answer,
    );
    println!("{}", answer);
}
