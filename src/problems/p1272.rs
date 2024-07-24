use std::io::Read;

fn dfs(
    tree: &Vec<Vec<usize>>,
    weight: &Vec<usize>,
    node: usize,
    parent: usize,
    special: usize,
    table: &mut Vec<Vec<usize>>,
) -> usize {
    if table[node][special] != 0 {
        return table[node][special];
    }

    let mut weight_c = weight[node] - weight[special];
    let mut weight_s = weight[node];

    for child in tree[node].iter() {
        if *child == parent {
            continue;
        }

        weight_c += dfs(tree, weight, *child, node, special, table);
        weight_s += dfs(tree, weight, *child, node, node, table);
    }

    table[node][special] = weight_s.min(weight_c);
    table[node][special]
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let root: usize = tokens.next().unwrap().parse().unwrap();

    let mut weight = vec![0; n];
    for i in 0..n {
        let w: usize = tokens.next().unwrap().parse().unwrap();
        weight[i] = w;
    }

    let mut tree = vec![vec![]; n];
    for _ in 0..n - 1 {
        let u: usize = tokens.next().unwrap().parse().unwrap();
        let v: usize = tokens.next().unwrap().parse().unwrap();
        tree[u - 1].push(v - 1);
        tree[v - 1].push(u - 1);
    }

    let mut table = vec![vec![0; n]; n];
    let mut answer = 0;
    for n in tree[root - 1].iter() {
        answer += dfs(&tree, &weight, *n, root - 1, root - 1, &mut table);
    }

    println!("{}", answer + weight[root - 1]);
}
