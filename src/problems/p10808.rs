pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let str = s.trim().chars().collect::<Vec<char>>();

    let mut cnt = [0; 26];
    for c in str {
        cnt[c as usize - 'a' as usize] += 1;
    }

    for i in 0..26 {
        print!("{} ", cnt[i]);
    }
}
