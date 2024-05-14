use std::fmt::Write;

#[derive(PartialEq, Clone)]
enum State {
    Unvisited,
    Visiting,
    Visited,
}

#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<usize>>,
}

fn dfs(graph: &Graph, v: usize, visited: &mut Vec<State>, stack: &mut Vec<usize>) -> bool {
    visited[v] = State::Visiting;
    for &u in &graph.adj[v] {
        if visited[u] == State::Visiting {
            return true;
        } else if visited[u] == State::Unvisited && dfs(graph, u, visited, stack) {
            return true;
        }
    }
    visited[v] = State::Visited;
    stack.push(v);
    return false;
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut line = input.split_ascii_whitespace();

    let n: usize = line.next().unwrap().parse().unwrap();
    let m: usize = line.next().unwrap().parse().unwrap();

    let mut graph = Graph {
        adj: vec![Vec::new(); n],
    };

    for _ in 0..m {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();

        line.windows(2).skip(1).for_each(|x| {
            graph.adj[x[0] - 1].push(x[1] - 1);
        });
    }

    let mut visited = vec![State::Unvisited; n];
    let mut stack = Vec::new();
    let mut has_cycle = false;
    for i in 0..n {
        if visited[i] == State::Unvisited && dfs(&graph, i, &mut visited, &mut stack) {
            has_cycle = true;
            break;
        }
    }

    if has_cycle {
        println!("0");
        return;
    }

    let mut output = String::new();
    stack
        .iter()
        .rev()
        .for_each(|x| writeln!(output, "{} ", x + 1).unwrap());
    println!("{}", output);
}
