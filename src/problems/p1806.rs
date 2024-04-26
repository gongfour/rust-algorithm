use std::io::Read;

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = iter.next().unwrap();
    let s = iter.next().unwrap();

    let mut d = iter.take(n).collect::<Vec<_>>();
    d.insert(0, 0);

    let mut p1 = 1;
    let mut p2 = 0;
    let mut d1 = d[0] + d[1];
    let mut d2 = d[0];

    let mut found = false;
    let mut output = n;
    while p1 != p2 {
        if d1 - d2 >= s {
            output = min(output, p1 - p2);
            p2 += 1;
            d2 += d[p2];
            found = true;
        } else {
            if p1 < n {
                p1 += 1;
                d1 += d[p1];
            } else {
                break;
            }
        }
    }
    print!("{}", if found { output } else { 0 });
}
