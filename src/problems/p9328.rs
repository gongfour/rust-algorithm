// Title: 열쇠
use std::collections::VecDeque;
use std::fmt;
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
        self.map[y][x] == '*'
    }

    fn is_key(&self, y: usize, x: usize) -> bool {
        self.map[y][x].is_lowercase()
    }

    fn is_door(&self, y: usize, x: usize) -> bool {
        self.map[y][x].is_uppercase()
    }

    fn is_document(&self, y: usize, x: usize) -> bool {
        self.map[y][x] == '$'
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn bfs(map: &Map, keys: &Vec<char>) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map.w]; map.h];
    let mut found = vec![vec![false; map.w]; map.h];
    let mut keys = keys.clone();
    let mut count = 0;
    queue.push_back((0, 0));

    while let Some((y, x)) = queue.pop_front() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

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

            if map.is_door(ny, nx) {
                let door = map.map[ny][nx];
                let door = door.to_lowercase().next().unwrap();
                if !keys.contains(&door) {
                    continue;
                }
            }

            if map.is_key(ny, nx) && !keys.contains(&map.map[ny][nx]) {
                let key = map.map[ny][nx];
                keys.push(key);
                visited = vec![vec![false; map.w]; map.h];
            }

            if map.is_document(ny, nx) && !found[ny][nx] {
                found[ny][nx] = true;
                count += 1;
            }

            queue.push_back((ny, nx));
        }
    }
    count
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    let t = iter.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let h = iter.next().unwrap().parse::<usize>().unwrap();
        let w = iter.next().unwrap().parse::<usize>().unwrap();

        let mut map = Map {
            map: vec![vec!['.'; w + 2]; h + 2],
            w: w + 2,
            h: h + 2,
        };
        for y in 0..h {
            let line = iter.next().unwrap().chars().collect::<Vec<char>>();
            for (x, c) in line.iter().enumerate() {
                map.map[y + 1][x + 1] = *c;
            }
        }
        let keys = iter.next().unwrap().chars().collect::<Vec<char>>();
        let ans = bfs(&map, &keys);
        println!("{}", ans);
    }
}
