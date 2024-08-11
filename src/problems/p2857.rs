pub fn main() {
    let mut found = false;
    for i in 1..6 {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        if s.contains("FBI") {
            print!("{} ", i);
            found = true;
        }
    }

    if !found {
        println!("HE GOT AWAY!");
    }
}
