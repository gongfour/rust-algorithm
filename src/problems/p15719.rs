use std::io::Read;

pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut tokens = s.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut is_visited = vec![false; n + 1];
    for _ in 0..n {
        let x: usize = tokens.next().unwrap().parse().unwrap();
        if is_visited[x] {
            println!("{}", x);
            return;
        }
        is_visited[x] = true;
    }
}
