use std::collections::VecDeque;

type Point = (usize, usize);

fn bfs(map: &Vec<Vec<char>>, s: Point, e: Point) -> usize {
    let mut q = VecDeque::new();
    q.push_back((s.0, s.1, 1));

    let n = map.len();
    let m = map[0].len();

    let mut v = vec![vec![false; m]; n];

    while let Some((y, x, c)) = q.pop_front() {
        if v[y][x] {
            continue;
        }
        if (y, x) == e {
            return c;
        }
        v[y][x] = true;

        let y = y as isize;
        let x = x as isize;
        let n = n as isize;
        let m = m as isize;

        let adjs = [
            ((y - 1).max(0), x),
            ((y + 1).min(n - 1), x),
            (y, (x - 1).max(0)),
            (y, (x + 1).min(m - 1)),
        ];

        for (ny, nx) in adjs.iter() {
            let ny = *ny as usize;
            let nx = *nx as usize;
            if map[ny][nx] == '1' {
                q.push_back((ny, nx, c + 1));
            }
        }
    }
    0
}

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut tokens = s.split_ascii_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let m = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut map = vec![];
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        map.push(s.trim().chars().collect::<Vec<char>>());
    }

    let ans = bfs(&map, (0, 0), (n - 1, m - 1));
    println!("{}", ans);
}
