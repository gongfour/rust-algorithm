use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let n = input.next().unwrap();
    let mut v: Vec<i32> = input.collect();

    let cutoff = n as f32 * 0.3 / 2.0;
    let cutoff = cutoff.round() as i32;

    v.sort();
    let t = &v[cutoff as usize..v.len() - cutoff as usize];
    let tsum = t.iter().sum::<i32>();
    let avg = tsum as f32 / t.len() as f32;
    let avg = avg.round() as i32;

    println!("{}", avg);
}

