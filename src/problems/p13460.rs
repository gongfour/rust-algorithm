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
        let mut goal_1 = false;
        let mut goal_2 = false;
        loop {
            let next_m1 = m1 + *d;
            let next_m2 = m2 + *d;

            let mut moved = false;
            if self.map[next_m1.y][next_m1.x] == 'O' {
                m1 = next_m1;
                goal_1 = true;
            }
            if self.map[next_m2.y][next_m2.x] == 'O' {
                m2 = next_m2;
                goal_2 = true;
            }
            if self.map[next_m1.y][next_m1.x] == '.'
                && !(next_m1.y == m2.y && next_m1.x == m2.x)
                && !goal_1
            {
                m1 = next_m1;
                moved = true;
            }
            if self.map[next_m2.y][next_m2.x] == '.'
                && !(next_m2.y == m1.y && next_m2.x == m1.x)
                && !goal_2
            {
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
        Direction { dy: -1, dx: 0 }, // 1. up
        Direction { dy: 1, dx: 0 },  // 2. down
        Direction { dy: 0, dx: -1 }, // 3. left
        Direction { dy: 0, dx: 1 },  // 4. right
    ];
    let lt = vec![vec![], vec![1, 2], vec![1, 2], vec![3, 4], vec![3, 4]];
    queue.push_back((red, blue, 0, vec![0]));

    while let Some((r, b, c, d)) = queue.pop_front() {
        // map.draw_map(&r, &b);
        // println!("d: {:?}, r: {}, b: {}", d, r, b);
        if c > 10 {
            return -1;
        }
        if map.map[b.y][b.x] == 'O' {
            continue;
        }
        if map.map[r.y][r.x] == 'O' {
            return c;
        }

        for i in 1..5 {
            let last = d.last().unwrap();
            if lt[*last].contains(&i) {
                continue;
            }
            let (nr, nb) = map.move_marble((r, b), &dirs[i]);
            if nr == r && nb == b {
                continue;
            }
            let mut dd = d.clone();
            dd.push(i);
            queue.push_back((nr, nb, c + 1, dd));
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

    // map.draw_map(&red, &blue);
    // (red, blue) = map.move_marble((red, blue), &Direction { dy: 0, dx: -1 });
    // map.draw_map(&red, &blue);
    // (red, blue) = map.move_marble((red, blue), &Direction { dy: 1, dx: 0 });
    // map.draw_map(&red, &blue);
    // (red, blue) = map.move_marble((red, blue), &Direction { dy: 0, dx: 1 });
    // map.draw_map(&red, &blue);
    // (red, blue) = map.move_marble((red, blue), &Direction { dy: 1, dx: 0 });
    // map.draw_map(&red, &blue);
    let ans = bfs(&mut map, red, blue);
    println!("{}", ans);
}
