pub fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let hex = s.trim();

    let map = ["000", "001", "010", "011", "100", "101", "110", "111"];

    if hex == "0" {
        println!("0");
        return;
    }

    let mut binary = String::new();
    for c in hex.chars() {
        match c.to_digit(8) {
            Some(d) => binary.push_str(map[d as usize]),
            None => (),
        }
    }

    println!("{}", binary.trim_start_matches('0'));
}
