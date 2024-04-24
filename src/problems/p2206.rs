use std::collections::VecDeque;
use std::io::Read;

struct Node {
    x: usize,
    y: usize,
    is_use: bool,
    dist: i32,
}

impl Node {
    fn new(y: usize, x: usize, is_use: bool, dist: i32) -> Self {
        Self { x, y, is_use, dist }
    }
}

struct Map {
    rows: usize,
    cols: usize,
    map: Vec<Vec<usize>>,
}

impl Map {
    fn new(n: usize, m: usize, map: Vec<Vec<usize>>) -> Self {
        Self {
            rows: n,
            cols: m,
            map,
        }
    }

    fn is_out(&self, y: i32, x: i32) -> bool {
        if x < 0 || y < 0 || y >= self.rows as i32 || x >= self.cols as i32 {
            return true;
        }
        false
    }
}

fn bfs(map: &Map, root: Node, end: Node) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![vec![false; map.cols]; map.rows]; 2];

    queue.push_back(root);

    while let Some(n) = queue.pop_front() {
        if n.x == end.x && n.y == end.y {
            return n.dist;
        }

        let visited = if !n.is_use {
            &mut visited[0]
        } else {
            &mut visited[1]
        };
        if visited[n.y][n.x] {
            continue;
        }
        visited[n.y][n.x] = true;

        let adj: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in adj {
            let nx = n.x as i32 + dx;
            let ny = n.y as i32 + dy;

            if map.is_out(ny, nx) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if map.map[ny][nx] == 0 {
                queue.push_back(Node::new(ny, nx, n.is_use, n.dist + 1));
            } else if !n.is_use {
                queue.push_back(Node::new(ny, nx, true, n.dist + 1));
            }
        }
    }
    return -1;
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<String>);

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut map = vec![vec![0 as usize; m]; n];
    for y in 0..n {
        let line = iter.next().unwrap().chars().collect::<Vec<char>>();
        for x in 0..m {
            map[y][x] = line[x].to_digit(10).unwrap() as usize;
        }
    }
    let map = Map::new(n, m, map);

    let output = bfs(
        &map,
        Node::new(0, 0, false, 1),
        Node::new(n - 1, m - 1, false, 1),
    );
    println!("{}", output);
}
