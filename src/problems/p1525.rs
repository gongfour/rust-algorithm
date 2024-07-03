use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::Read;

fn bfs(answer_map: &Vec<i32>, map: &Vec<i32>) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((map.clone(), 0));
    let mut visited = HashSet::new();

    while let Some((cur, c)) = q.pop_front() {
        // println!("{:?} {c}", cur);
        // println!("v{:?}", visited);
        if cur == *answer_map {
            return c;
        }

        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur.clone());

        let zero = cur.iter().position(|&x| x == 0).unwrap();
        let x = zero / 3;
        let y = zero % 3;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= 3 || ny < 0 || ny >= 3 {
                continue;
            }

            let nzero = nx as usize * 3 + ny as usize;
            let mut nmap = cur.clone();
            nmap.swap(zero, nzero);

            q.push_back((nmap, c + 1));
        }
    }
    -1
}

pub fn main() {
    let answer_map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let map = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let ans = bfs(&answer_map, &map);
    println!("{}", ans);
}
