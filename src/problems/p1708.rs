use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn angle(a: &Point, b: &Point) -> f64 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dy as f64).atan2(dx as f64)
}

fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut points = Vec::new();
    for _ in 0..n {
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        points.push(Point { x, y });
    }

    points.sort_by(|a, b| {
        if a.y == b.y {
            a.x.cmp(&b.x)
        } else {
            a.y.cmp(&b.y)
        }
    });
    let start = points[0];
    // println!("{:?}", &start);

    points.sort_by(|a, b| angle(&start, a).partial_cmp(&angle(&start, b)).unwrap());
    // println!("{:?}", &points);

    let mut stack = Vec::new();
    stack.push(points[0]);
    stack.push(points[1]);

    for p in points.iter().skip(2) {
        while stack.len() >= 2 {
            let a = stack[stack.len() - 2];
            let b = stack[stack.len() - 1];
            if ccw(&a, &b, &p) > 0 {
                break;
            }
            stack.pop();
        }
        stack.push(*p);
    }

    // for (i, p) in stack.iter().enumerate() {
    //     println!("{}: {:?}", i, p);
    // }
    // println!("len: {}", stack.len());
    println!("{}", stack.len());
}
