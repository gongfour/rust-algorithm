// Title: 줄 세우기

use std::fmt::Write;
use std::io::Read;

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }
}

fn dfs(graph: &Graph, v: usize, visited: &mut Vec<bool>, output: &mut Vec<usize>) {
    visited[v] = true;

    for &u in &graph.adj[v] {
        if !visited[u] {
            dfs(graph, u, visited, output);
        }
    }
    output.push(v);
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut graph = Graph::new(n);

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        graph.adj[a - 1].push(b - 1);
    }

    let mut stack = vec![];
    let mut visited = vec![false; n];

    for i in 0..n {
        if !visited[i] {
            dfs(&mut graph, i, &mut visited, &mut stack);
        }
    }

    let mut output = String::new();
    while let Some(v) = stack.pop() {
        write!(output, "{} ", v + 1);
    }
    println!("{}", output);
}
