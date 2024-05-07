use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let d = s
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    if d[0] == 60 && d[1] == 60 && d[2] == 60 {
        println!("Equilateral");
    } else if d[0] + d[1] + d[2] == 180 {
        if d[0] == d[1] || d[1] == d[2] || d[2] == d[0] {
            println!("Isosceles");
        } else {
            println!("Scalene");
        }
    } else {
        println!("Error");
    }
}
