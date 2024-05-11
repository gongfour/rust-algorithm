use std::fmt::Write;

type B = [u8; 81];

fn is_valid(d: &B, y: usize, x: usize, n: usize) -> bool {
    for r in 0..9 {
        if d[y * 9 + r] == n as u8 {
            return false;
        }
    }

    for c in 0..9 {
        if d[c * 9 + x] == n as u8 {
            return false;
        }
    }

    let sy = y / 3 * 3;
    let sx = x / 3 * 3;
    for r in 0..3 {
        for c in 0..3 {
            if d[(sy + r) * 9 + sx + c] == n as u8 {
                return false;
            }
        }
    }

    true
}

fn dfs(d: &mut B, solved: &mut bool) {
    if *solved {
        return;
    }
    if !d.contains(&0) {
        print_board(d);
        *solved = true;
        return;
    }

    for y in 0..9 {
        for x in 0..9 {
            if d[y * 9 + x] == 0 {
                for n in 1..=9 {
                    if is_valid(d, y, x, n) {
                        d[y * 9 + x] = n as u8;
                        dfs(d, solved);
                        d[y * 9 + x] = 0;
                    }
                }
                return;
            }
        }
    }
}

fn print_board(d: &B) {
    let mut s = String::new();
    for y in 0..9 {
        for x in 0..9 {
            write!(s, "{}", d[y * 9 + x]).unwrap();
        }
        writeln!(s).unwrap();
    }
    println!("{}", s);
}

pub fn main() {
    let mut d = [0; 81];
    for v in 0..9 {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        for (i, c) in s.trim().chars().enumerate() {
            d[v * 9 + i] = c as u8 - '0' as u8;
        }
    }
    dfs(&mut d, &mut false);
}
