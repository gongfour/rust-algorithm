use std::io::Read;

type Board = Vec<Vec<isize>>;
type Queen = (usize, usize);

enum State {
    Put,
    Pop,
}

fn put_or_pop_queen(b: &mut Board, q: &Queen, state: State) {
    let (y, x) = q;
    let n = b.len() as isize;

    let m = match state {
        State::Put => 1,
        State::Pop => -1,
    };

    b[*y][*x] += m;

    let y = *y as isize;
    let x = *x as isize;
    for i in 1..n {
        let ny_add = y + i;
        let nx_add = x + i;
        let ny_sub = y - i;
        let nx_sub = x - i;
        if ny_add < n && nx_add < n {
            b[ny_add as usize][nx_add as usize] += m;
        }
        if ny_add < n && nx_sub >= 0 {
            b[ny_add as usize][nx_sub as usize] += m;
        }
        if ny_sub >= 0 && nx_add < n {
            b[ny_sub as usize][nx_add as usize] += m;
        }
        if ny_sub >= 0 && nx_sub >= 0 {
            b[ny_sub as usize][nx_sub as usize] += m;
        }
    }
}

fn backtrack(queens: &Vec<Queen>, i: usize, b: &mut Board, c: usize) -> usize {
    if i == queens.len() {
        return c;
    }

    let q = queens[i];

    let mut max_c = 0;
    if b[q.0][q.1] == 0 {
        put_or_pop_queen(b, &q, State::Put);
        max_c = backtrack(queens, i + 1, b, c + 1);
        put_or_pop_queen(b, &q, State::Pop);
    }

    max_c = max_c.max(backtrack(queens, i + 1, b, c));
    max_c
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut queens_b = vec![];
    let mut queens_w = vec![];

    for y in 0..n {
        for x in 0..n {
            if tokens.next().unwrap() == "1" {
                if (y + x) % 2 == 0 {
                    queens_b.push((y, x));
                } else {
                    queens_w.push((y, x));
                }
            }
        }
    }

    let mut b_b = vec![vec![0; n]; n];
    let mut b_w = vec![vec![0; n]; n];
    let count_b = backtrack(&queens_b, 0, &mut b_b, 0);
    let count_w = backtrack(&queens_w, 0, &mut b_w, 0);
    println!("{}", count_b + count_w);
}
