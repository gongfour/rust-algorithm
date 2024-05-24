use std::io::Read;

type Position = (usize, usize); // (y, x)

fn dfs(map: &Vec<Vec<u8>>, s: Position, visited: &mut Vec<Vec<bool>>, count: &mut u32) {
    let n = map.len();
    let (y, x) = s;

    if visited[y][x] || map[y][x] == 0 {
        return;
    }
    visited[y][x] = true;
    *count += 1;

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if nx < 0 || nx >= n as i32 || ny < 0 || ny >= n as i32 {
            continue;
        }
        dfs(map, (ny as usize, nx as usize), visited, count);
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut m = vec![vec![0; n]; n];
    for y in 0..n {
        let line = tokens.next().unwrap();
        for (x, c) in line.chars().enumerate() {
            m[y][x] = c.to_digit(10).unwrap() as u8;
        }
    }

    let mut visited = vec![vec![false; n]; n];
    let mut areas = vec![];
    let mut count = 0;
    for y in 0..n {
        for x in 0..n {
            if visited[y][x] || m[y][x] == 0 {
                continue;
            }
            count += 1;
            let mut area = 0;
            dfs(&m, (y, x), &mut visited, &mut area);
            areas.push(area);
        }
    }
    areas.sort();
    println!("{}", count);
    areas.iter().for_each(|a| println!("{}", a));
}
