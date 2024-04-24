use std::io::{stdin, Read};

fn dfs(map: &Vec<Vec<usize>>, visited: &mut Vec<Vec<bool>>, y: usize, x: usize) {
    let mut stack = vec![(y, x)];
    visited[y][x] = true;

    while let Some((cy, cx)) = stack.pop() {
        if cy > 0 && map[cy - 1][cx] == 1 && !visited[cy - 1][cx] {
            stack.push((cy - 1, cx));
            visited[cy - 1][cx] = true;
        }
        if cy < map.len() - 1 && map[cy + 1][cx] == 1 && !visited[cy + 1][cx] {
            stack.push((cy + 1, cx));
            visited[cy + 1][cx] = true;
        }
        if cx > 0 && map[cy][cx - 1] == 1 && !visited[cy][cx - 1] {
            stack.push((cy, cx - 1));
            visited[cy][cx - 1] = true;
        }
        if cx < map[0].len() - 1 && map[cy][cx + 1] == 1 && !visited[cy][cx + 1] {
            stack.push((cy, cx + 1));
            visited[cy][cx + 1] = true;
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let mut t = iter.next().unwrap();

    while t > 0 {
        t = t - 1;
        let m = iter.next().unwrap();
        let n = iter.next().unwrap();
        let k = iter.next().unwrap();
        let mut map = vec![vec![0 as usize; m]; n];

        for _ in 0..k {
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            map[y][x] = 1;
        }

        let mut visited = vec![vec![false; m]; n];
        let mut count = 0;
        for y in 0..n {
            for x in 0..m {
                if map[y][x] == 1 && !visited[y][x] {
                    dfs(&map, &mut visited, y, x);
                    count += 1;
                }
            }
        }
        println!("{}", count);
    }
}
