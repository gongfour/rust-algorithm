// Title: 트리 순회 순서 변경
//

use std::fmt;
use std::fmt::Write;

struct Tree(Vec<Node>);

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Node {
    value: char,
    left: Option<usize>,
    right: Option<usize>,
}
impl Node {
    fn new(value: char, left: Option<usize>, right: Option<usize>) -> Self {
        Node { value, left, right }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}: {} {})",
            self.value,
            index_to_char(self.left),
            index_to_char(self.right)
        )
    }
}

fn char_to_index(c: char) -> usize {
    c as usize - 'A' as usize
}

fn index_to_char(i: Option<usize>) -> char {
    match i {
        None => '.',
        Some(i) => (i as u8 + 'A' as u8) as char,
    }
}

fn dfs(
    tree: &Vec<Node>,
    s: usize,
    visited: &mut Vec<bool>,
    preorder: &mut Vec<char>,
    inorder: &mut Vec<char>,
    postorder: &mut Vec<char>,
) {
    visited[s] = true;
    preorder.push(tree[s].value);

    if let Some(left) = tree[s].left {
        if !visited[left] {
            dfs(tree, left, visited, preorder, inorder, postorder);
        }
    }
    inorder.push(tree[s].value);

    if let Some(right) = tree[s].right {
        if !visited[right] {
            dfs(tree, right, visited, preorder, inorder, postorder);
        }
    }
    postorder.push(tree[s].value);
}

fn print_answer(stack: &Vec<char>) {
    let mut output = String::new();
    for c in stack {
        write!(output, "{}", c).unwrap();
    }
    println!("{}", output);
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut tree = vec![Node::new('.', None, None); n];
    for _ in 0..n {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let line = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<char>().unwrap())
            .collect::<Vec<char>>();

        let index = char_to_index(line[0]);
        tree[index] = Node::new(
            line[0],
            if line[1] == '.' {
                None
            } else {
                Some(char_to_index(line[1]))
            },
            if line[2] == '.' {
                None
            } else {
                Some(char_to_index(line[2]))
            },
        );
    }

    let mut preorder = vec![];
    let mut inorder = vec![];
    let mut postorder = vec![];
    dfs(
        &tree,
        0,
        &mut vec![false; n],
        &mut preorder,
        &mut inorder,
        &mut postorder,
    );
    print_answer(&preorder);
    print_answer(&inorder);
    print_answer(&postorder);
}
