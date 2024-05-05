use std::io::Read;

#[derive(Clone, Copy)]
struct Direction {
    dy: i32,
    dx: i32,
}

struct Map {
    map: Vec<Vec<char>>,
    n: usize,
    m: usize,
}

impl Map {
    fn new(n: usize, m: usize) -> Self {
        Map {
            map: vec![vec!['.'; m]; n],
            n,
            m,
        }
    }

    fn draw(&mut self, pos: (usize, usize), c: char) {
        self.map[pos.0][pos.1] = c;
    }

    fn draw_map(&self, red: &Marble, blue: &Marble) {
        let mut map = self.map.clone();
        map[red.y][red.x] = 'R';
        map[blue.y][blue.x] = 'B';

        for y in 0..self.n {
            for x in 0..self.m {
                print!("{}", map[y][x]);
            }
            println!();
        }
    }

    fn move_marble(&mut self, ms: (Marble, Marble), d: &Direction) -> (Marble, Marble) {
        let mut m1 = ms.0;
        let mut m2 = ms.1;
        loop {
            let next_m1 = m1 + *d;
            let next_m2 = m2 + *d;

            if self.map[next_m1.y][next_m1.x] == 'O' {
                return (next_m1, next_m2);
            }
            if self.map[next_m2.y][next_m2.x] == 'O' {
                return (next_m1, next_m2);
            }

            let mut moved = false;
            if self.map[next_m1.y][next_m1.x] == '.' {
                m1 = next_m1;
                moved = true;
            }
            if self.map[next_m2.y][next_m2.x] == '.' {
                m2 = next_m2;
                moved = true;
            }

            if !moved {
                return (m1, m2);
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.n {
            for x in 0..self.m {
                write!(f, "{}", self.map[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Marble {
    y: usize,
    x: usize,
}

impl std::ops::Add<Direction> for Marble {
    type Output = Marble;

    fn add(self, dir: Direction) -> Marble {
        let nx = self.x as i32 + dir.dx;
        let ny = self.y as i32 + dir.dy;

        Marble {
            y: ny as usize,
            x: nx as usize,
        }
    }
}

impl std::fmt::Display for Marble {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

fn bfs(map: &mut Map, red: Marble, blue: Marble) -> i32 {
    let mut queue = std::collections::VecDeque::new();
    let dirs = [
        Direction { dy: 0, dx: 0 },  // stay
        Direction { dy: -1, dx: 0 }, // up
        Direction { dy: 1, dx: 0 },  // down
        Direction { dy: 0, dx: -1 }, // left
        Direction { dy: 0, dx: 1 },  // right
    ];
    let lt = vec![vec![], vec![1, 2], vec![1, 2], vec![3, 4], vec![3, 4]];
    queue.push_back((red, blue, 0, 0));

    while let Some((r, b, c, d)) = queue.pop_front() {
        println!("d: {}, c: {}", d, c);
        println!("r: {}, b: {}", r, b);
        map.draw_map(&r, &b);
        if c > 2 {
            return -1;
        }
        if map.map[b.y][b.x] == 'O' {
            continue;
        }
        if map.map[r.y][r.x] == 'O' {
            return c;
        }

        for i in 1..5 {
            if lt[d].contains(&i) {
                continue;
            }
            let (r, b) = map.move_marble((r, b), &dirs[i]);
            queue.push_back((r, b, c + 1, i));
        }
    }
    -1
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut map = Map::new(n, m);

    let mut red = Marble { y: 0, x: 0 };
    let mut blue = Marble { y: 0, x: 0 };
    for y in 0..n {
        let line = iter.next().unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == 'R' {
                red = Marble { y, x };
            } else if c == 'B' {
                blue = Marble { y, x };
            } else {
                map.map[y][x] = c;
            }
        }
    }

    println!("{}", map);
    let ans = bfs(&mut map, red, blue);
    println!("{}", ans);
}
