use std::collections::VecDeque;
struct Node {
    x: usize,
    y: usize,
    is_use: bool,
    dist: usize,
}

impl Node {
    fn new(x: usize, y: usize, is_use: bool, dist: usize) -> Self {
        Self { x, y, is_use, dist }
    }
}

fn bfs(map: &Vec<Vec<usize>>, root: Node, end: Node) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut visited_ = vec![vec![false; map[0].len()]; map.len()];

    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        let x = node.x;
        let y = node.y;
        let b = node.is_use;
        let d = node.dist;
        if x == end.x && y == end.y {
            return d as i32;
        }
        if !b && visited[y][x] {
            continue;
        }
        if b && visited_[y][x] {
            continue;
        }

        if !b {
            visited[y][x] = true;
        } else {
            visited_[y][x] = true;
        }

        if y > 0 {
            if map[y - 1][x] == 0 {
                queue.push_back(Node::new(x, y - 1, b, d + 1));
            } else if !b {
                queue.push_back(Node::new(x, y - 1, true, d + 1));
            }
        }
        if y < map.len() - 1 {
            if map[y + 1][x] == 0 {
                queue.push_back(Node::new(x, y + 1, b, d + 1));
            } else if !b {
                queue.push_back(Node::new(x, y + 1, true, d + 1));
            }
        }
        if x > 0 {
            if map[y][x - 1] == 0 {
                queue.push_back(Node::new(x - 1, y, b, d + 1));
            } else if !b {
                queue.push_back(Node::new(x - 1, y, true, d + 1));
            }
        }
        if x < map[0].len() - 1 {
            if map[y][x + 1] == 0 {
                queue.push_back(Node::new(x + 1, y, b, d + 1));
            } else if !b {
                queue.push_back(Node::new(x + 1, y, true, d + 1));
            }
        }
    }
    return -1;
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut map = vec![vec![]; n];

    for i in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        line.trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .for_each(|x| {
                map[i].push(x as usize);
            });
    }

    let output = bfs(
        &map,
        Node::new(0, 0, false, 1),
        Node::new(m - 1, n - 1, false, 1),
    );
    println!("{}", output);
}
