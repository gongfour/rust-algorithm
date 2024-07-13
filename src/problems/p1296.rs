use std::io::Read;

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let name = tokens.next().unwrap();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut teams = vec![];
    for _ in 0..n {
        let team = tokens.next().unwrap();
        teams.push(team);
    }

    let mut nl = 0;
    let mut no = 0;
    let mut nv = 0;
    let mut ne = 0;

    for c in name.chars() {
        match c {
            'L' => nl += 1,
            'O' => no += 1,
            'V' => nv += 1,
            'E' => ne += 1,
            _ => (),
        }
    }

    let mut result = vec![];

    for team in teams {
        let mut tl = 0;
        let mut to = 0;
        let mut tv = 0;
        let mut te = 0;
        for c in team.chars() {
            match c {
                'L' => tl += 1,
                'O' => to += 1,
                'V' => tv += 1,
                'E' => te += 1,
                _ => (),
            }
        }
        let l = nl + tl;
        let o = no + to;
        let v = nv + tv;
        let e = ne + te;

        let score = ((l + o) * (l + v) * (l + e) * (o + v) * (o + e) * (v + e)) % 100;
        result.push((score, team));
    }

    result.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    println!("{}", result[0].1);
}
