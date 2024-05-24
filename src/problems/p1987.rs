use std::io::Read;

type Position = (usize, usize);
type Path = Vec<bool>;
fn backtrack(map: &Vec<Vec<char>>, pos: Position, path: &mut Path, count: &mut u32, index: u32) {
    let (y, x) = pos;
    let c = map[y][x] as usize - 'A' as usize;

    if path[c] {
        return;
    }
    path[c] = true;

    if index > *count {
        *count = index;
    }

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for dir in directions {
        let (dx, dy) = dir;
        let ny = y as i32 + dy;
        let nx = x as i32 + dx;

        if nx < 0 || nx >= map[0].len() as i32 || ny < 0 || ny >= map.len() as i32 {
            continue;
        }

        backtrack(map, (ny as usize, nx as usize), path, count, index + 1);
    }
    path[c] = false;
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let r: usize = tokens.next().unwrap().parse().unwrap();
    let c: usize = tokens.next().unwrap().parse().unwrap();
    let mut m = vec![vec!['0'; c]; r];
    for y in 0..r {
        let line = tokens.next().unwrap().trim();
        for (x, c) in line.chars().enumerate() {
            m[y][x] = c;
        }
    }

    let mut paths = vec![false; 26];
    let mut count = 0;
    backtrack(&m, (0, 0), &mut paths, &mut count, 1);
    println!("{}", count);
}
