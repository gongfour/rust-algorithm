use std::collections::VecDeque;
use std::io::Read;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.find(self.parent[x]);
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        self.parent[x] = y;
    }
}

fn bfs(g: &Vec<usize>, s: usize, v: &mut Vec<bool>, uf: &mut UnionFind) {
    let mut queue = VecDeque::new();
    queue.push_back(s);

    while let Some(u) = queue.pop_front() {
        let n = g[u];
        if v[n] {
            continue;
        }
        v[n] = true;
        queue.push_back(n);
        uf.union(u, n);
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut g = vec![0; n * m];
    for y in 0..n {
        let line = iter.next().unwrap().chars();
        for (x, c) in line.enumerate() {
            let i = y * m + x;
            match c {
                'D' => g[i] = i + m,
                'U' => g[i] = i - m,
                'L' => g[i] = i - 1,
                'R' => g[i] = i + 1,
                _ => (),
            }
        }
    }

    let mut uf = UnionFind::new(n * m);
    let mut visited = vec![false; n * m];
    for y in 0..n {
        for x in 0..m {
            let i = y * m + x;
            if visited[g[i]] {
                uf.union(i, g[i]);
                continue;
            }
            if visited[i] {
                continue;
            }
            bfs(&g, i, &mut visited, &mut uf);
        }
    }

    let mut ans: Vec<usize> = (0..n * m).map(|i| uf.find(i)).collect();
    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
}
