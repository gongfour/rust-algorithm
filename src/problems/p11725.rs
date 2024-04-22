// Title: 트리의 부모 찾기

use std::{
    fmt::Write,
    io::{stdin, Read},
};

struct Node {
    edges: Vec<usize>,
}

impl Node {
    fn new() -> Node {
        Node { edges: Vec::new() }
    }

    fn add_edge(&mut self, edge: usize) {
        self.edges.push(edge);
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(n: usize) -> Graph {
        let mut nodes = Vec::with_capacity(n);
        for _ in 0..n {
            nodes.push(Node::new());
        }
        Graph { nodes }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].add_edge(v);
        self.nodes[v].add_edge(u);
    }
}

fn bfs(tree: &mut Graph, root: usize) -> Vec<usize> {
    let mut parents = vec![0; tree.nodes.len()];
    let mut queue = Vec::new();
    queue.push(root);

    while !queue.is_empty() {
        let u = queue.pop().unwrap();
        for &v in &tree.nodes[u].edges {
            if parents[v] == 0 {
                parents[v] = u + 1;
                queue.push(v);
            }
        }
    }

    parents
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<usize>());

    let n = iter.next().unwrap();
    let mut tree = Graph::new(n);
    for _ in 0..n - 1 {
        let e1 = iter.next().unwrap() - 1;
        let e2 = iter.next().unwrap() - 1;

        tree.add_edge(e1, e2);
    }
    // println!("{:?}", tree);
    let result = bfs(&mut tree, 0);
    let mut output = String::new();
    for i in 1..n {
        write!(output, "{}\n", result[i]).unwrap();
    }

    println!("{}", output);
}
