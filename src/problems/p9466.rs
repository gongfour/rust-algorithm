use std::io::Read;

#[derive(Clone)]
struct Student {
    want: usize,
}

impl Student {
    fn new() -> Self {
        Self { want: 0 }
    }
}

fn dfs(students: &mut Vec<Student>, start: usize) -> (usize, Vec<usize>) {
    let mut stack = vec![start];
    let mut visited = vec![false; students.len() + 1];
    let mut count = 0;
    let mut team = vec![];

    while let Some(mut cur) = stack.pop() {
        if visited[cur] && start == cur {
            return (count, team);
        } else if visited[cur] {
            return (0, vec![]);
        }
        count += 1;
        team.push(cur);
        visited[cur] = true;
        stack.push(students[cur].want);
    }
    (0, vec![])
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let t = iter.next().unwrap();

    for _ in 0..t {
        let n = iter.next().unwrap();
        let mut students = vec![Student::new(); n + 1];
        for i in 1..=n {
            let want = iter.next().unwrap();
            students[i].want = want;
        }

        let mut sum = 0;
        let mut visited = vec![false; n + 1];
        for i in 1..=n {
            if !visited[i] {
                let s = dfs(&mut students, i);
                sum += s.0;
                for j in s.1 {
                    visited[j] = true;
                }
            }
        }
        println!("{}", n - sum);
    }
}
