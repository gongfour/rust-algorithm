use std::isize;

pub fn main() {
    let mut binary_string = String::new();
    std::io::stdin().read_line(&mut binary_string).unwrap();

    let mut padded_binary_string = binary_string.trim().to_string();
    while padded_binary_string.len() % 3 != 0 {
        padded_binary_string = format!("0{}", padded_binary_string);
    }

    for window in padded_binary_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
    {
        let window: String = window.iter().collect();
        let digit = isize::from_str_radix(&window, 2).unwrap();
        print!("{:o}", digit);
    }
}
