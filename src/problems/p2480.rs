fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut n = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    if n[0] == n[1] && n[1] == n[2] {
        println!("{}", 10000 + n[0] * 1000);
    } else if n[0] == n[1] {
        println!("{}", 1000 + n[0] * 100);
    } else if n[1] == n[2] {
        println!("{}", 1000 + n[1] * 100);
    } else if n[0] == n[2] {
        println!("{}", 1000 + n[0] * 100);
    } else {
        println!("{}", n.iter().max().unwrap() * 100);
    }
}