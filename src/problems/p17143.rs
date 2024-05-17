use std::io::Read;

struct Shark {
    i: usize,
    y: usize,
    x: usize,
    s: usize,
    d: usize,
    z: usize,
}

impl Shark {
    fn flip(&mut self) {
        match self.d {
            1 => self.d = 2,
            2 => self.d = 1,
            3 => self.d = 4,
            4 => self.d = 3,
            _ => unreachable!(),
        }
    }

    fn step(&mut self) {
        match self.d {
            1 => self.y -= 1,
            2 => self.y += 1,
            3 => self.x += 1,
            4 => self.x -= 1,
            _ => unreachable!(),
        }
    }

    fn check_bound(&self, r: usize, c: usize) -> bool {
        match self.d {
            1 => self.y == 1,
            2 => self.y == r,
            3 => self.x == c,
            4 => self.x == 1,
            _ => unreachable!(),
        }
    }

    fn move_shark(&mut self, r: usize, c: usize) -> (usize, usize) {
        let speed = match self.d {
            1 | 2 => self.s % (2 * r - 2),
            3 | 4 => self.s % (2 * c - 2),
            _ => unreachable!(),
        };
        for _ in 0..speed {
            if self.check_bound(r, c) {
                self.flip();
            }
            self.step();
        }
        (self.y, self.x)
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let r: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut sharks = Vec::new();
    for i in 0..m {
        let y: usize = iter.next().unwrap().parse().unwrap();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let s: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();
        let z: usize = iter.next().unwrap().parse().unwrap();

        sharks.push(Some(Shark { i, y, x, s, d, z }));
    }
    let mut weight = 0;
    for x in 1..=c {
        // Catch shark
        let shark = sharks
            .iter_mut()
            .filter_map(|opt| opt.as_ref())
            .filter(|shark| shark.x == x)
            .min_by(|s1, s2| s1.y.cmp(&s2.y));

        match shark {
            Some(shark) => {
                weight += shark.z;
                let index = shark.i;
                sharks[index] = None;
            }
            None => {}
        }

        // Move sharks
        let mut board: Vec<Vec<Option<usize>>> = vec![vec![None; c]; r];
        for i in 0..sharks.len() {
            match &sharks[i] {
                Some(_) => {
                    sharks[i].as_mut().unwrap().move_shark(r, c);
                    let y = sharks[i].as_ref().unwrap().y;
                    let x = sharks[i].as_ref().unwrap().x;
                    let i_b = board[y - 1][x - 1];

                    if i_b == None {
                        board[y - 1][x - 1] = Some(i);
                        continue;
                    }

                    if sharks[i_b.unwrap()].is_none() {
                        board[y - 1][x - 1] = Some(i);
                        continue;
                    }

                    let shark1 = sharks[i].as_ref().unwrap();
                    let shark2 = sharks[i_b.unwrap()].as_ref().unwrap();

                    if shark1.z < shark2.z {
                        sharks[i] = None;
                        board[y - 1][x - 1] = i_b;
                    } else {
                        sharks[i_b.unwrap()] = None;
                        board[y - 1][x - 1] = Some(i);
                    }
                }
                None => {}
            }
        }
    }
    println!("{}", weight);
}
