use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let d = iter.map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let mut dd = d.clone();
    dd.sort();
    dd.dedup();

    let ed: Vec<(usize, &i32)> = dd.iter().enumerate().collect();

    // println!("{:?}", ed);

    for x in d.iter() {
        let i = ed.binary_search_by(|a| a.1.cmp(x)).unwrap();
        // println!("{} {} ", x, i);
        print!("{} ", i);
    }
}
