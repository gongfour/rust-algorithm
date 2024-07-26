use std::collections::VecDeque;
use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let t1: usize = tokens.next().unwrap().parse().unwrap();
    let t2: usize = tokens.next().unwrap().parse().unwrap();
    let t1 = t1 - 1;
    let t2 = t2 - 1;
    let num_e: usize = tokens.next().unwrap().parse().unwrap();

    let mut adj = vec![vec![]; n];
    for _ in 0..num_e {
        let u: usize = tokens.next().unwrap().parse().unwrap();
        let v: usize = tokens.next().unwrap().parse().unwrap();
        adj[u - 1].push(v - 1);
        adj[v - 1].push(u - 1);
    }

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back((t1, 0));

    while let Some((u, c)) = queue.pop_front() {
        if u == t2 {
            println!("{}", c);
            return;
        }
        if visited[u] {
            continue;
        }
        visited[u] = true;

        for edge in adj[u].iter() {
            if !visited[*edge] {
                queue.push_back((*edge, c + 1));
            }
        }
    }
    println!("{}", -1);
}
