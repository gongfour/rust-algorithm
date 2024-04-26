use std::io::Read;

fn solve(s: i32, cnt: &mut usize, d: &Vec<i32>, i: usize, sum: i32, c: usize) {
    if i == d.len() {
        if sum == s && c != 0 {
            *cnt += 1;
        }
        return;
    }

    solve(s, cnt, d, i + 1, sum, c);
    solve(s, cnt, d, i + 1, sum + d[i], c + 1);
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let s = iter.next().unwrap().parse::<i32>().unwrap();

    let d = iter.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let mut count = 0;
    solve(s, &mut count, &d, 0, 0, 0);
    println!("{}", count);
}
