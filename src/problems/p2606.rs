use std::collections::VecDeque;
use std::io::{stdin, Read, Write};

#[derive(Debug, Clone)]
struct Graph {
    verteces: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Self {
            verteces: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.verteces[u].push(v);
        self.verteces[v].push(u);
    }
}

fn bfs(graph: &Graph, start: usize) -> Vec<usize> {
    let n = graph.verteces.len();
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start] = true;

    let mut result = vec![];
    while let Some(u) = queue.pop_front() {
        result.push(u);

        for &v in &graph.verteces[u] {
            if !visited[v] {
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }

    result
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    // println!("{} {:?}", n, m);
    let mut d = vec![];
    for _ in 0..m {
        let u = iter.next().unwrap() - 1;
        let v = iter.next().unwrap() - 1;
        d.push((u, v));
    }

    let mut g = Graph::new(n);
    d.iter().for_each(|&(u, v)| {
        g.add_edge(u, v);
    });
    // println!("{:?}", g);

    let result = bfs(&g, 0);
    println!("{:?}", result.len() - 1);
}
