use std::collections::VecDeque;
use std::io::Read;

struct Map {
    map: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

impl Map {
    fn is_out(&self, y: i32, x: i32) -> bool {
        y >= self.h as i32 || x >= self.w as i32 || y < 0 || x < 0
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        self.map[y][x] == '#'
    }

    fn is_key(&self, y: usize, x: usize) -> bool {
        self.map[y][x].is_lowercase()
    }

    fn is_door(&self, y: usize, x: usize) -> bool {
        self.map[y][x].is_uppercase()
    }

    fn is_exit(&self, y: usize, x: usize) -> bool {
        self.map[y][x] == '1'
    }
}

fn has_key(keys: usize, key: char) -> bool {
    keys & (1 << (key as usize - 'a' as usize)) != 0
}

fn add_key(keys: usize, key: char) -> usize {
    keys | (1 << (key as usize - 'a' as usize))
}

fn bfs(map: &Map, start: (usize, usize)) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![vec![false; map.w]; map.h]; 64];
    queue.push_back(((start), 0, 0));

    while let Some(((y, x), c, k)) = queue.pop_front() {
        if visited[k][y][x] {
            continue;
        }
        visited[k][y][x] = true;

        if map.is_exit(y, x) {
            return c;
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if map.is_out(ny, nx) {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;

            if map.is_wall(ny, nx) {
                continue;
            }

            let ch = map.map[ny][nx];
            if map.is_door(ny, nx) {
                let door = ch;
                let door = door.to_lowercase().next().unwrap();
                if !has_key(k, door) {
                    continue;
                }
            }

            let mut new_k = k;
            if map.is_key(ny, nx) && !has_key(k, ch) {
                new_k = add_key(k, ch);
            }

            queue.push_back(((ny, nx), c + 1, new_k));
        }
    }
    -1
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    let h = iter.next().unwrap().parse::<usize>().unwrap();
    let w = iter.next().unwrap().parse::<usize>().unwrap();

    let mut map = Map {
        map: vec![vec!['.'; w]; h],
        w,
        h,
    };
    let mut start = (0, 0);
    for y in 0..h {
        let line = iter.next().unwrap().chars().collect::<Vec<char>>();
        for (x, c) in line.iter().enumerate() {
            if *c == '0' {
                map.map[y][x] = '.';
                start = (y, x);
                continue;
            } else {
                map.map[y][x] = *c;
            }
        }
    }
    let ans = bfs(&map, start);
    println!("{}", ans);
}
