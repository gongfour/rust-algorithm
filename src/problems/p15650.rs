fn dfs(n: i32, m: i32, stack: &mut Vec<i32>) {
    if stack.len() == m as usize {
        for i in stack {
            print!("{} ", i);
        }
        println!();
        return;
    }

    for i in 1..=n {
        if stack.len() == 0 || stack.last().unwrap() < &i {
            stack.push(i);
            dfs(n, m, stack);
            stack.pop();
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.split_whitespace().flat_map(str::parse::<i32>);

    let n = tokens.next().unwrap();
    let m = tokens.next().unwrap();

    let mut stack = Vec::new();
    dfs(n, m, &mut stack);
}
