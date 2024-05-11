use std::fmt::Write;

fn print_stack(output: &mut String, stack: &Vec<usize>) {
    for i in stack {
        write!(output, "{} ", i).unwrap();
    }
    writeln!(output).unwrap();
}

fn dfs(
    n: i32,
    m: i32,
    d: &Vec<usize>,
    visited: &mut Vec<bool>,
    stack: &mut Vec<usize>,
    answer: &mut String,
) {
    if stack.len() == m as usize {
        print_stack(answer, stack);
    }

    for (i, v) in d.iter().enumerate() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        stack.push(*v);
        dfs(n, m, d, visited, stack, answer);
        stack.pop();
        visited[i] = false;
    }
}


pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace().flat_map(str::parse::<i32>);

    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut d = input.split_whitespace().flat_map(str::parse::<usize>).collect::<Vec<usize>>();

    d.sort();
    let mut stack = Vec::new();
    let mut visited = vec![false; n as usize];

    let mut answer = String::new();
    dfs(n, m, &d, &mut visited, &mut stack, &mut answer);
    println!("{}", answer);
}
