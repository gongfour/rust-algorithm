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
    output: &mut Vec<(usize, usize)>,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(yx);
    let mut area = 0;

    while let Some((y, x)) = queue.pop_front() {
        if visited[y][x] || board[y][x] == 1 {
            continue;
        }
        area += 1;
        visited[y][x] = true;

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
                queue.push_back((ny, nx));
            } else {
                output.push((ny, nx));
            }
        }
    }
    area
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

    let mut board_count = Board::new(n, m, 0);
    let mut visited = Board::new(n, m, false);
    for y in 0..n {
        for x in 0..m {
            if board[y][x] != 0 || visited[y][x] {
                continue;
            }

            let mut adjs = Vec::new();
            let area = bfs(&board, (y, x), &mut visited, &mut adjs);
            adjs.sort();
            adjs.dedup();

            for (ny, nx) in &adjs {
                board_count[*ny][*nx] += area;
            }
        }
    }

    let mut board_output = Board::new(n, m, 0 as u8);
    for y in 0..n {
        for x in 0..m {
            if board[y][x] == 1 {
                board_output[y][x] = ((board_count[y][x] + 1) % 10) as u8;
            }
        }
    }
    println!("{}", board_output);
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
