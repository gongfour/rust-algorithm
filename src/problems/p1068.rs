use std::io::Read;

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Node { edges: Vec::new() }
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new(n: usize) -> Self {
        Tree {
            nodes: vec![Node::new(); n],
        }
    }

    fn add(&mut self, parent: usize, child: usize) {
        self.nodes[parent].edges.push(child);
    }
}

fn dfs(tree: &Tree, n: usize, r: usize, v: &mut Vec<bool>, count: &mut usize) {
    v[n] = true;
    if n == r {
        return;
    }
    let edges = &tree.nodes[n].edges;
    if edges.len() == 1 && edges[0] == r || edges.is_empty() {
        *count += 1;
        return;
    }
    for &u in &tree.nodes[n].edges {
        if !v[u] {
            dfs(tree, u, r, v, count);
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut tree = Tree::new(n);
    let mut start = 0;
    for i in 0..n {
        let parent: i32 = tokens.next().unwrap().parse().unwrap();
        if parent == -1 {
            start = i;
            continue;
        }
        tree.add(parent as usize, i);
    }
    let r = tokens.next().unwrap().parse().unwrap();
    // println!("{:?}", tree);
    // println!("start: {}", start);
    // println!("r: {}", r);

    let mut count = 0;
    dfs(&tree, start, r, &mut vec![false; n], &mut count);
    println!("{}", count);
}
