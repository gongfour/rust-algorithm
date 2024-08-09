use std::io::Read;

fn swap(arr: &mut Vec<usize>, a: usize, b: usize) {
    let tmp = arr[a];
    arr[a] = arr[b];
    arr[b] = tmp;
}

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_whitespace();

    let m: usize = tokens.next().unwrap().parse().unwrap();

    let mut cup = vec![1, 2, 3];
    for _ in 0..m {
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let b: usize = tokens.next().unwrap().parse().unwrap();

        let pa = cup.iter().position(|&x| x == a).unwrap();
        let pb = cup.iter().position(|&x| x == b).unwrap();

        swap(&mut cup, pa, pb);
    }

    println!("{}", cup[0]);
}
