use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();

    let mut mat = vec![vec![0; m]; n];
    for y in 0..n {
        for x in 0..m {
            mat[y][x] = tokens.next().unwrap().parse().unwrap();
        }
    }

    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut problems = Vec::new();
    for _ in 0..k {
        let sx: usize = tokens.next().unwrap().parse().unwrap();
        let sy: usize = tokens.next().unwrap().parse().unwrap();
        let ex: usize = tokens.next().unwrap().parse().unwrap();
        let ey: usize = tokens.next().unwrap().parse().unwrap();
        problems.push((sx, sy, ex, ey));
    }

    let mut dp = vec![vec![0 as i64; m]; n];
    dp[0][0] = mat[0][0];
    for y in 1..n {
        dp[y][0] = dp[y - 1][0] + mat[y][0];
    }
    for x in 1..m {
        dp[0][x] = dp[0][x - 1] + mat[0][x];
    }
    for y in 1..n {
        for x in 1..m {
            dp[y][x] = dp[y - 1][x] + dp[y][x - 1] - dp[y - 1][x - 1] + mat[y][x];
        }
    }

    for (sx, sy, ex, ey) in problems {
        let sx = sx - 1;
        let sy = sy - 1;
        let ex = ex - 1;
        let ey = ey - 1;
        let mut result = dp[ex][ey];
        if sx > 0 {
            result -= dp[sx - 1][ey];
        }
        if sy > 0 {
            result -= dp[ex][sy - 1];
        }
        if sx > 0 && sy > 0 {
            result += dp[sx - 1][sy - 1];
        }
        println!("{}", result);
    }
}

fn print_mat(mat: &Vec<Vec<i64>>) {
    for row in mat {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}
