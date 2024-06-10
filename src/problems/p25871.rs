use std::io::Read;

fn can_enter(v: usize, d: &Vec<usize>) -> bool {
    let mut v = v;
    if v == 0 {
        return !d.contains(&0);
    }

    while v > 0 {
        let digit = v % 10;
        if d.contains(&digit) {
            return false;
        }
        v /= 10;
    }
    true
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut d = vec![0; n];
    for i in 0..n {
        d[i] = tokens.next().unwrap().parse::<usize>().unwrap();
    }

    let m = tokens.next().unwrap().parse::<usize>().unwrap();

    // println!("{:?}", n);
    // println!("{:?}", d);
    // println!("{:?}", m);

    let mut min_answer = usize::MAX;
    for c in 0..=999 {
        // println!("{:?}", c);
        if can_enter(c, &d) {
            let answer = m as isize - c as isize;
            let answer = answer.abs();
            min_answer = min_answer.min(answer as usize);
        }
    }
    println!("{}", min_answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_enter_channel() {
        assert_eq!(can_enter(100, &vec![0]), false);
        assert_eq!(can_enter(250, &vec![3, 4]), true);
        assert_eq!(can_enter(100, &vec![1, 2, 3]), false);
        assert_eq!(can_enter(100, &vec![2, 3, 4]), true);
    }
}
