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

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }

    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parent[y] = x;
        }
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let v = iter.next().unwrap();
    let e = iter.next().unwrap();

    let mut edges = vec![];
    for _ in 0..e {
        let a = iter.next().unwrap() - 1;
        let b = iter.next().unwrap() - 1;
        let c = iter.next().unwrap();

        edges.push((c, a, b));
    }

    edges.sort();
    let mut uf = UnionFind::new(v as usize);
    // let mut tree = vec![];
    let mut sum = 0;
    for (c, a, b) in edges {
        let ra = uf.find(a as usize);
        let rb = uf.find(b as usize);
        if ra != rb {
            uf.union(ra, rb);
            // tree.push((c, a, b));
            sum += c;
        }
    }

    println!("{}", sum);
}
