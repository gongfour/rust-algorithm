use std::collections::VecDeque;
use std::fmt::Write;
use std::io::Read;

fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> String {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    queue.push_back(s);

    let mut output = String::new();

    while let Some(v) = queue.pop_front() {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        for &u in &graph[v] {
            queue.push_back(u);
        }
        write!(output, "{} ", v + 1).unwrap();
    }
    return output;
}

fn dfs(graph: &Vec<Vec<usize>>, s: usize, visited: &mut Vec<bool>, output: &mut String) {
    if visited[s] {
        return;
    }
    visited[s] = true;
    write!(output, "{} ", s + 1).unwrap();

    for &u in &graph[s] {
        dfs(graph, u, visited, output);
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace().flat_map(str::parse::<usize>);

    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();
    let s = tokens.next().unwrap() - 1;

    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        let u = tokens.next().unwrap();
        let v = tokens.next().unwrap();
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    for i in 0..n {
        graph[i].sort();
    }

    // println!("n{} m{} s{}", n, m, s);
    // println!("{:?}", graph);

    let mut dfs_answer = String::new();
    dfs(&graph, s, &mut vec![false; n], &mut dfs_answer);
    println!("{}", dfs_answer);
    let bfs_answer = bfs(&graph, s);
    println!("{}", bfs_answer);
}
