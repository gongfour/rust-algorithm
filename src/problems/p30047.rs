pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut s = input.trim().chars().collect::<Vec<_>>();

    let mut stack = Vec::new();
    let mut valid = true;
    while let Some(c) = s.pop() {
        // println!("{}: {:?}", c, s);
        match c {
            'x' => stack.push(0),
            'g' => {
                let v = stack.pop();
                match v {
                    Some(v) => stack.push(v + 1),
                    None => valid = false,
                }
            }
            'f' => {
                let a = stack.pop();
                let b = stack.pop();

                match (a, b) {
                    (Some(a), Some(b)) => stack.push(a.min(b)),
                    _ => valid = false,
                }
            }
            _ => unreachable!("invalid character: {}", c),
        };
    }

    if stack.len() != 1 {
        valid = false;
    }

    if valid {
        println!("{}", stack.pop().unwrap());
    } else {
        println!("-1");
    }
}
