fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<u32>().unwrap();

    let finger = n % 8;
    let ans = match finger {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 4,
        7 => 3,
        0 => 2,
        _ => 0,
    };
    println!("{}", ans);
}
