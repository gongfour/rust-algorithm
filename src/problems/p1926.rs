use std::io::Read;

fn dfs(map: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, sy: isize, sx: isize, area: &mut usize) {
    let n = map.len();
    let m = map[0].len();

    if sy < 0
        || sx < 0
        || sy >= n as isize
        || sx >= m as isize
        || visited[sy as usize][sx as usize]
        || map[sy as usize][sx as usize] == 0
    {
        return;
    }

    visited[sy as usize][sx as usize] = true;
    *area += 1;

    dfs(map, visited, sy + 1, sx, area);
    dfs(map, visited, sy - 1, sx, area);
    dfs(map, visited, sy, sx + 1, area);
    dfs(map, visited, sy, sx - 1, area);
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let m = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut map = vec![vec![0; m]; n];

    for y in 0..n {
        for x in 0..m {
            map[y][x] = tokens.next().unwrap().parse::<u8>().unwrap();
        }
    }

    let mut visited = vec![vec![false; m]; n];
    let mut count = 0;
    let mut maximum_area = 0;
    for y in 0..n {
        for x in 0..m {
            if visited[y][x] || map[y][x] == 0 {
                continue;
            }
            count += 1;
            let mut area = 0;
            dfs(&map, &mut visited, y as isize, x as isize, &mut area);
            maximum_area = maximum_area.max(area);
        }
    }

    println!("{}\n{}", count, maximum_area);
}
