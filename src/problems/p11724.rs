use std::io::{stdin, Read};

struct Graph {
    nodes: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            nodes: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].push(v);
        self.nodes[v].push(u);
    }
}

fn dfs(graph: &Graph, visited: &mut Vec<bool>, s: usize) {
    let mut queue = vec![s];

    while let Some(u) = queue.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for &v in &graph.nodes[u] {
            if !visited[v] {
                queue.push(v);
            }
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut graph = Graph::new(n);
    for _ in 0..m {
        let u = iter.next().unwrap() - 1;
        let v = iter.next().unwrap() - 1;
        graph.add_edge(u, v);
    }

    let mut visited = vec![false; graph.nodes.len()];
    let mut count = 0;
    for i in 0..n {
        if !visited[i] {
            dfs(&graph, &mut visited, i);
            count += 1;
        }
    }

    println!("{}", count);
}
