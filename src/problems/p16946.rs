// Title: 벽 부수고 이동하기 4

use std::collections::VecDeque;
use std::fmt;
use std::ops::{Index, IndexMut};

struct Board<T>(Vec<Vec<T>>, usize, usize);

impl<T> Board<T> {
    fn new(n: usize, m: usize, fill: T) -> Self
    where
        T: Clone,
    {
        Board(vec![vec![fill; m]; n], n, m)
    }

    fn height(&self) -> usize {
        self.1
    }

    fn width(&self) -> usize {
        self.2
    }
}

fn bfs(
    board: &Board<u8>,
    yx: (usize, usize),
    visited: &mut Board<bool>,
    output: &mut Vec<(usize, usize, usize)>,
) {
    let mut queue = VecDeque::new();
    queue.push_back((yx.0, yx.1, 0));

    while let Some((y, x, d)) = queue.pop_front() {
        if visited[y][x] || board[y][x] == 1 {
            continue;
        }
        visited[y][x] = true;
        output.push((y, x, d));

        let y = y as i32;
        let x = x as i32;
        let h = board.height() as i32;
        let w = board.width() as i32;

        let nexts = [
            (y, (x - 1).max(0)),
            (y, (x + 1).min(w - 1)),
            ((y - 1).max(0), x),
            ((y + 1).min(h - 1), x),
        ];

        for (ny, nx) in nexts.iter() {
            let ny = *ny as usize;
            let nx = *nx as usize;

            if board[ny][nx] == 0 {
                queue.push_back((ny, nx, d + 1));
            }
        }
    }
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else {
            self.parent[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();

    let mut board = Board::new(n, m, 0);

    for i in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let tokens = input.trim().chars();
        for (j, c) in tokens.enumerate() {
            board[i][j] = c.to_digit(10).unwrap() as u8;
        }
    }

    let mut uf = UnionFind::new(n * m);
    let mut board_count = Board::new(n, m, 0);
    let mut visited = Board::new(n, m, false);
    for y in 0..n {
        for x in 0..m {
            if board[y][x] != 0 || visited[y][x] {
                continue;
            }

            let mut output = Vec::new();
            bfs(&board, (y, x), &mut visited, &mut output);

            for (ny, nx, _) in &output {
                uf.union(y * m + x, *ny * m + *nx);
                board_count[*ny][*nx] = output.len();
            }
        }
    }
    // println!("{:?}", uf);

    let mut board_output = Board::new(n, m, 0);
    for y in 0..n {
        for x in 0..m {
            if board[y][x] != 1 {
                continue;
            }

            let y = y as i32;
            let x = x as i32;

            let mut count = 1 as usize;
            let adjs = [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)];
            let mut adjs = adjs
                .iter()
                .filter(|(ny, nx)| {
                    *ny >= 0
                        && *nx >= 0
                        && *ny < n as i32
                        && *nx < m as i32
                        && board[*ny as usize][*nx as usize] == 0
                })
                .map(|(ny, nx)| uf.find(*ny as usize * m + *nx as usize))
                .collect::<Vec<_>>();

            adjs.sort();
            adjs.dedup();

            for adj in adjs.iter() {
                let key = *adj as usize;
                count += board_count[key / m][key % m] as usize;
            }

            let y = y as usize;
            let x = x as usize;

            board_output[y][x] = (count % 10) as u8;
        }
    }
    // println!("{}", board_count);
    // println!("{}", board);
    // println!("{}", board_output);

    for row in &board_output.0 {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

impl<T: fmt::Display> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Index<usize> for Board<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Board<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
