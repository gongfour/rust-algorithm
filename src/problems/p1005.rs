// Title: ACM Craft
// URL: https://www.acmicpc.net/problem/1005
// Date: 2021-07-03

use std::collections::VecDeque;
use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().flat_map(str::parse::<usize>);

    let t = iter.next().unwrap();

    for _ in 0..t {
        let n = iter.next().unwrap();
        let k = iter.next().unwrap();
        let mut d = vec![0; n];
        let mut dp = vec![0; n];
        let mut graph = vec![vec![]; n];
        for i in 0..n {
            d[i] = iter.next().unwrap();
        }
        // println!("{:?}", d);
        for _ in 0..k {
            let x = iter.next().unwrap() - 1;
            let y = iter.next().unwrap() - 1;
            graph[y].push(x);
        }

        let mut candidate = vec![];

        let w = iter.next().unwrap() - 1;
        let mut queue = VecDeque::new();
        queue.push_back(w);
        dp[w] = d[w];

        // println!("{}: {:?}", w + 1, dp);
        while let Some(x) = queue.pop_front() {
            if graph[x].is_empty() {
                candidate.push(x);
            }

            for &y in &graph[x] {
                if dp[y] >= dp[x] + d[y] {
                    continue;
                }
                dp[y] = dp[x] + d[y];
                queue.push_back(y);
            }
            // println!("{}: {:?}", x + 1, dp);
        }

        let mut answer = 0;
        for i in candidate {
            answer = answer.max(dp[i]);
        }
        println!("{}", answer);
    }
}
