use std::collections::VecDeque;

struct Node {
    x: usize,
    y: usize,
    is_use: bool,
    dist: usize,
}

impl Node {
    fn new(y: usize, x: usize, is_use: bool, dist: usize) -> Self {
        Self { x, y, is_use, dist }
    }
}

fn bfs(map: &Vec<Vec<usize>>, root: Node, end: Node) -> i32 {
    let n = map.len();
    let m = map[0].len();

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];
    let mut visited_b = vec![vec![false; m]; n];

    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        let x = node.x;
        let y = node.y;
        let b = node.is_use;
        let d = node.dist;
        if x == end.x && y == end.y {
            return d as i32;
        }

        let visited = if !b { &mut visited } else { &mut visited_b };
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        let adj: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        for (dx, dy) in adj {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || ny >= n as i32 || nx >= m as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if map[y][x] == 0 {
                queue.push_back(Node::new(ny, nx, b, d + 1));
            } else if !b {
                queue.push_back(Node::new(ny, nx, true, d + 1));
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
        Node::new(n - 1, m - 1, false, 1),
    );
    println!("{}", output);
}
