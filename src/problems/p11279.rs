use std::fmt::Write;
use std::io::Read;

struct Heap {
    data: Vec<usize>,
}

impl Heap {
    fn new() -> Self {
        Heap { data: Vec::new() }
    }
    fn parent(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn right(&self, i: usize) -> usize {
        2 * i + 2
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    fn push(&mut self, x: usize) {
        self.data.push(x);
        let mut i = self.data.len() - 1;
        while i > 0 {
            let p = self.parent(i);
            if self.data[p] < self.data[i] {
                self.swap(p, i);
                i = p;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<usize> {
        if self.data.len() == 0 {
            return None;
        }

        if self.data.len() == 1 {
            return self.data.pop();
        }

        let ret = self.data[0];
        self.data[0] = self.data.pop().unwrap();
        let mut i = 0;
        while i < self.data.len() {
            let l = self.left(i);
            let r = self.right(i);
            let next = if r < self.data.len() {
                if self.data[l] < self.data[r] {
                    r
                } else {
                    l
                }
            } else if l < self.data.len() {
                l
            } else {
                break;
            };

            if self.data[i] < self.data[next] {
                self.swap(i, next);
                i = next;
            } else {
                break;
            }
        }
        Some(ret)
    }

    fn print(&self) {
        for i in 0..self.data.len() {
            print!("{} ", self.data[i]);
        }
        println!();
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();

    let mut heap = Heap::new();
    let mut output = String::new();
    for _ in 0..n {
        let v = iter.next().unwrap();

        if v == 0 {
            match heap.pop() {
                Some(x) => write!(output, "{}\n", x).unwrap(),
                None => write!(output, "0\n").unwrap(),
            };
        } else {
            heap.push(v);
        }
    }
    println!("{}", output);
}
