fn bfs(a: u64, b: u64, c: u64) -> i32 {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((a, b, c));
    while let Some((na, _, nc)) = queue.pop_front() {
        if na == b {
            return nc as i32;
        }
        if na * 2 <= b {
            queue.push_back((na * 2, b, nc + 1));
        }
        if na * 10 + 1 <= b {
            queue.push_back((na * 10 + 1, b, nc + 1));
        }
    }
    -1
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: u64 = iter.next().unwrap().parse().unwrap();
    let b: u64 = iter.next().unwrap().parse().unwrap();

    let n = bfs(a, b, 1);
    println!("{}", n);
}
