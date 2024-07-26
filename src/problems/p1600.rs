use std::collections::VecDeque;
use std::io::Read;

type Position = (usize, usize);

struct Map {
    map: Vec<Vec<usize>>,
    n: usize,
    m: usize,
}

impl Map {
    fn new(n: usize, m: usize) -> Map {
        Map {
            map: vec![vec![0; m]; n],
            n: n,
            m: m,
        }
    }

    fn is_out_of_bound(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= self.m as i32 || y >= self.n as i32
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        self.map[y][x] == 1
    }
}

fn bfs(map: &Map, start: &Position, end: &Position, k: usize) -> i32 {
    let mut visited = vec![vec![vec![false; map.m]; map.n]; k + 1];
    let mut q = VecDeque::new();
    q.push_back((*start, 0, 0));

    let adj = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        // for horse
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
    ];

    while let Some(((x, y), c, ck)) = q.pop_front() {
        if visited[ck][y][x] {
            continue;
        }
        visited[ck][y][x] = true;

        if (x, y) == *end {
            return c;
        }

        for (i, dir) in adj.iter().enumerate() {
            let (dx, dy) = dir;
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if map.is_out_of_bound(nx, ny) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if map.is_wall(nx, ny) {
                continue;
            }

            if i >= 4 && ck >= k {
                continue;
            }

            if i >= 4 {
                q.push_back(((nx, ny), c + 1, ck + 1));
            } else {
                q.push_back(((nx, ny), c + 1, ck));
            }
        }
    }
    -1
}

fn print_map(map: &Map) {
    for y in 0..map.n {
        for x in 0..map.m {
            print!("{}", map.map[y][x]);
        }
        println!();
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let k: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let n: usize = tokens.next().unwrap().parse().unwrap();

    let mut map = Map::new(n, m);
    for y in 0..n {
        for x in 0..m {
            let c: usize = tokens.next().unwrap().parse().unwrap();
            map.map[y][x] = c;
        }
    }

    let answer = bfs(&map, &(0, 0), &(m - 1, n - 1), k);
    println!("{}", answer);
}
