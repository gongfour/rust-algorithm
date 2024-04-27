use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut d = iter.map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    d.sort();

    let mut ans_l = 0;
    let mut ans_m = 1;
    let mut ans_r = n - 1;
    let mut min = d[ans_l] + d[ans_m] + d[ans_r];

    for l in 0..n - 2 {
        let mut m = l + 1;
        let mut r = n - 1;
        while m < r {
            let s = d[l] + d[m] + d[r];

            if min.abs() > s.abs() {
                min = s;
                ans_l = l;
                ans_m = m;
                ans_r = r;
            }

            if s < 0 {
                m += 1;
            } else {
                r -= 1;
            }
        }
    }

    println!("{} {} {}", d[ans_l], d[ans_m], d[ans_r]);
}
