use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Read;

fn bfs(ladder: &HashMap<usize, usize>, start: usize, end: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; 101];
    queue.push_back((start, 0));

    while let Some((x, c)) = queue.pop_front() {
        if visited[x] {
            continue;
        }
        visited[x] = true;

        for i in 1..=6 {
            let y = x + i;
            if y >= end {
                return Some(c + 1);
            }
            if let Some(&z) = ladder.get(&y) {
                queue.push_back((z, c + 1));
            } else {
                queue.push_back((y, c + 1));
            }
        }
    }
    None
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut ladder = HashMap::new();
    for _ in 0..n + m {
        let x = iter.next().unwrap().parse::<usize>().unwrap();
        let y = iter.next().unwrap().parse::<usize>().unwrap();
        ladder.insert(x, y);
    }

    let ans = bfs(&ladder, 1, 100).unwrap();
    println!("{}", ans);
}
