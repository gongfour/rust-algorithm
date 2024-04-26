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

fn dfs(students: &mut Vec<Student>, visited: &mut Vec<bool>, start: usize) -> Vec<usize> {
    let mut stack = vec![start];
    let mut team = vec![];

    while let Some(cur) = stack.pop() {
        team.push(cur);
        if visited[cur] {
            return team;
        }
        visited[cur] = true;
        stack.push(students[cur].want);
    }
    team
}

// [4, 7, 6, 4]
fn count_cycle(s: &mut Vec<usize>) -> usize {
    let mut sum = 0;
    s.reverse();
    let iter = s.iter().skip(1);
    for v in iter {
        sum += 1;
        if *v == s[0] {
            return sum;
        }
    }
    0
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

        let mut visited = vec![false; n + 1];
        let mut sum = 0;
        for i in 1..=n {
            if !visited[i] {
                let mut s = dfs(&mut students, &mut visited, i);
                sum += count_cycle(&mut s);
            }
        }
        println!("{}", n - sum);
    }
}
