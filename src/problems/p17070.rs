use std::io;

type Map = Vec<Vec<u8>>;
type Position = (usize, usize);
type State = (Position, Direction);

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Diagonal,
}

fn dfs(map: &Map, state: State, count: &mut usize) {
    let ((ey, ex), dir) = state;

    if ey == map.len() - 1 && ex == map.len() - 1 {
        // Goal
        *count += 1;
        return;
    }

    let dydx: Vec<(i32, i32, Direction)> = match dir {
        Direction::Right => vec![(0, 1, Direction::Right), (1, 1, Direction::Diagonal)],
        Direction::Down => vec![(1, 0, Direction::Down), (1, 1, Direction::Diagonal)],
        Direction::Diagonal => vec![
            (0, 1, Direction::Right),
            (1, 0, Direction::Down),
            (1, 1, Direction::Diagonal),
        ],
    };

    for (dy, dx, dir) in dydx.iter() {
        let (ny, nx) = (ey as i32 + dy, ex as i32 + dx);
        if ny < 0 || nx < 0 || ny >= map.len() as i32 || nx >= map.len() as i32 {
            // Out of range
            continue;
        }
        if map[ny as usize][nx as usize] == 1 {
            // Wall
            continue;
        }
        if *dy == 1 && *dx == 1 {
            if map[ey + 1][ex] == 1 || map[ey][ex + 1] == 1 {
                // Wall
                continue;
            }
        }
        dfs(map, ((ny as usize, nx as usize), *dir), count);
    }
}

pub fn main() {
    let input = io::read_to_string(std::io::stdin()).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut board = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            board[i][j] = tokens.next().unwrap().parse::<u8>().unwrap();
        }
    }

    let mut count = 0;
    dfs(&board, ((0, 1), Direction::Right), &mut count);
    println!("{}", count);
}
