use std::collections::{vec_deque, VecDeque};
use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<usize>().unwrap();
    let d = tokens
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    let mut index: VecDeque<usize> = (1..=n).collect();
    let mut index_array = VecDeque::new();

    for i in d {
        match i {
            1 => {
                let card = index.pop_front().unwrap();
                index_array.push_front(card);
                card
            }
            2 => {
                let temp = index.pop_front().unwrap();
                let card = index.pop_front().unwrap();
                index.push_front(temp);
                index_array.push_front(card);
                card
            }
            3 => {
                let card = index.pop_back().unwrap();
                index_array.push_front(card);
                card
            }
            _ => unreachable!(),
        };
    }
    let mut answer: Vec<(usize, &usize)> = index_array.iter().enumerate().collect();
    answer.sort_by(|a, b| a.1.cmp(b.1));
    answer
        .iter()
        .map(|(a, _)| a + 1)
        .for_each(|x| print!("{} ", x));
}
