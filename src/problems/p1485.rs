use std::fmt::Write;
use std::io::Read;

fn is_square(dists: Vec<i32>) -> bool {
    let mut dists = dists.clone();
    dists.sort();
    dists[0] == dists[1] && dists[1] == dists[2] && dists[2] == dists[3] && dists[4] == dists[5]
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let t: usize = tokens.next().unwrap().parse().unwrap();
    let mut output = String::new();
    for _ in 0..t {
        let mut points = vec![];
        for _ in 0..4 {
            let x: i32 = tokens.next().unwrap().parse().unwrap();
            let y: i32 = tokens.next().unwrap().parse().unwrap();
            points.push((x, y));
        }

        let mut dists = vec![];
        for i in 0..4 {
            for j in i + 1..4 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                dists.push((x1 - x2).pow(2) + (y1 - y2).pow(2));
            }
        }
        if is_square(dists) {
            writeln!(output, "1").unwrap();
        } else {
            writeln!(output, "0").unwrap();
        }
    }
    print!("{}", output);
}
