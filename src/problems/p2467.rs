use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let d = iter.map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let mut l = 0;
    let mut r = n - 1;

    let mut ans_l = l;
    let mut ans_r = r;

    let mut min = d[l] + d[r];

    while l < r {
        let s = d[l] + d[r];
        if min.abs() > s.abs() {
            min = s;
            ans_l = l;
            ans_r = r;
        }

        if s < 0 {
            l += 1;
        } else {
            r -= 1;
        }
    }

    println!("{} {}", d[ans_l], d[ans_r]);
}
