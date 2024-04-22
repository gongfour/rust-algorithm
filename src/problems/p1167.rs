#[derive(Clone)]
struct Node {
    edge: Vec<(usize, usize)>,
}

impl Node {
    fn new() -> Self {
        Node { edge: Vec::new() }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new(n: usize) -> Self {
        Tree {
            nodes: vec![Node::new(); n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: usize) {
        self.nodes[u].edge.push((v, w));
        // self.nodes[v].edge.push((u, w));
    }
}

fn dfs(tree: &Tree, s: usize) -> Vec<usize> {
    let mut stack = vec![(s, 0 as usize)];
    let mut visited = vec![false; tree.nodes.len()];
    let mut values = vec![0; tree.nodes.len()];
    visited[s] = true;
    while let Some((u, g)) = stack.pop() {
        for &(v, w) in &tree.nodes[u].edge {
            if !visited[v] {
                visited[v] = true;
                values[v] = g + w;
                stack.push((v, g + w));
            }
        }
    }
    values
}

fn max_value(values: &Vec<usize>) -> (usize, usize) {
    let mut max = 0;
    let mut idx = 0;
    for (i, &v) in values.iter().enumerate() {
        if v > max {
            max = v;
            idx = i;
        }
    }
    (idx, max)
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut tree = Tree::new(n);
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let d = input
            .split_ascii_whitespace()
            .flat_map(str::parse::<usize>)
            .collect::<Vec<usize>>();

        let u = d[0] - 1;
        for j in 0..(d.len() - 1) / 2 {
            let v = d[2 * j + 1] - 1;
            let w = d[2 * j + 2];
            tree.add_edge(u, v, w);
        }
    }
    let values = dfs(&tree, 0);
    let (idx1, _) = max_value(&values);
    let values = dfs(&tree, idx1);
    let (_, dist2) = max_value(&values);
    println!("{}", dist2);
}
