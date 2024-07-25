use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut arr = vec![(0, 0); n + 1];
    for i in 1..=n {
        let x: usize = tokens.next().unwrap().parse().unwrap();
        arr[i] = (x, i);
    }

    let mut sorted_arr = arr.clone();
    sorted_arr.sort();

    let mut max_distance = 0;
    for i in 1..=n {
        let distance = sorted_arr[i].1 as i32 - i as i32;
        max_distance = max_distance.max(distance);
    }

    println!("{:?}", sorted_arr);
    println!("{}", max_distance + 1);
}
