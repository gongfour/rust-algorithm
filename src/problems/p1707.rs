use std::io::Read;

fn is_bipartite(adj: &Vec<Vec<usize>>) -> bool {
    let mut visited = vec![0; adj.len()];
    let mut stack = vec![];
    for i in 1..adj.len() {
        if visited[i] != 0 {
            continue;
        }
        stack.push(i);
        visited[i] = 1;
        while let Some(v) = stack.pop() {
            for &u in &adj[v] {
                if visited[u] == 0 {
                    // not visited
                    // assign opposite color
                    visited[u] = -visited[v];
                    stack.push(u);
                } else if visited[u] == visited[v] {
                    // visited and same color
                    return false;
                }
            }
        }
    }
    true
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let t: usize = tokens.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let num_v: usize = tokens.next().unwrap().parse().unwrap();
        let num_e: usize = tokens.next().unwrap().parse().unwrap();

        let mut adj = vec![vec![]; num_v + 1];
        for _ in 0..num_e {
            let v: usize = tokens.next().unwrap().parse().unwrap();
            let u: usize = tokens.next().unwrap().parse().unwrap();
            adj[v].push(u);
            adj[u].push(v);
        }
        println!("{}", if is_bipartite(&adj) { "YES" } else { "NO" });
    }
}
