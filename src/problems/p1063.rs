use std::io::Read;

fn str_to_pos(s: &str) -> (i32, i32) {
    let mut pos = s.chars();
    let x = pos.next().unwrap();
    let y = pos.next().unwrap();

    (x as i32 - 'A' as i32 + 1, y as i32 - '1' as i32 + 1)
}

fn pos_to_str(pos: (i32, i32)) -> String {
    let x = (pos.0 + 'A' as i32 - 1) as u8 as char;
    let y = (pos.1 + '1' as i32 - 1) as u8 as char;

    format!("{}{}", x, y)
}

fn move_object(pos: (i32, i32), dir: &str) -> Option<(i32, i32)> {
    let mut pos = match dir {
        "R" => (pos.0 + 1, pos.1),
        "L" => (pos.0 - 1, pos.1),
        "B" => (pos.0, pos.1 - 1),
        "T" => (pos.0, pos.1 + 1),
        "RT" => (pos.0 + 1, pos.1 + 1),
        "LT" => (pos.0 - 1, pos.1 + 1),
        "RB" => (pos.0 + 1, pos.1 - 1),
        "LB" => (pos.0 - 1, pos.1 - 1),
        _ => panic!("Invalid move"),
    };
    if pos.0 < 1 || pos.0 > 8 || pos.1 < 1 || pos.1 > 8 {
        return None;
    }
    Some(pos)
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let str_king = tokens.next().unwrap().trim().to_string();
    let mut pos_king = str_to_pos(&str_king);
    let str_stone = tokens.next().unwrap().trim().to_string();
    let mut pos_stone = str_to_pos(&str_stone);

    let n = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut moves = Vec::new();
    for _ in 0..n {
        moves.push(tokens.next().unwrap().trim());
    }

    for m in moves {
        let next_pos_king = move_object(pos_king, m);
        match next_pos_king {
            Some(next_pos_king) => {
                if next_pos_king == pos_stone {
                    let next_pos_stone = move_object(pos_stone, m);
                    match next_pos_stone {
                        Some(next_pos_stone) => {
                            pos_stone = next_pos_stone;
                            pos_king = next_pos_king;
                        }
                        None => {
                            continue;
                        }
                    }
                } else {
                    pos_king = next_pos_king;
                }
            }
            None => {
                continue;
            }
        }
    }

    let ans_king = pos_to_str(pos_king);
    let ans_stone = pos_to_str(pos_stone);

    println!("{}", ans_king);
    println!("{}", ans_stone);
}
