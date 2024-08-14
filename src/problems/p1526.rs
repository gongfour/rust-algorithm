pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();
    for i in (1..n).rev() {
        let num = i.to_string();
        if num.chars().all(|c| c == '4' || c == '7') {
            println!("{}", i);
            return;
        }
    }
}
