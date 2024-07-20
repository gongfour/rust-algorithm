use std::io::Read;

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 => 31,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => panic!("Invalid month"),
    }
}

fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    let mut day_of_year = day;
    for m in 1..month {
        day_of_year += days_in_month(year, m);
    }
    day_of_year
}

fn d_day(date: (i32, i32, i32), target: (i32, i32, i32)) -> i32 {
    let (year, month, day) = date;
    let (target_year, target_month, target_day) = target;

    if year == target_year {
        return day_of_year(target_year, target_month, target_day) - day_of_year(year, month, day);
    }
    let mut d_day = 0;

    d_day += (if is_leap_year(year) { 366 } else { 365 }) - day_of_year(year, month, day);

    for year in year + 1..target_year {
        d_day += if is_leap_year(year) { 366 } else { 365 };
    }

    d_day += day_of_year(target_year, target_month, target_day);

    d_day
}

pub fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let current_year: usize = tokens.next().unwrap().parse().unwrap();
    let current_month: usize = tokens.next().unwrap().parse().unwrap();
    let current_day: usize = tokens.next().unwrap().parse().unwrap();

    let target_year: usize = tokens.next().unwrap().parse().unwrap();
    let target_month: usize = tokens.next().unwrap().parse().unwrap();
    let target_day: usize = tokens.next().unwrap().parse().unwrap();

    let d_day = d_day(
        (
            current_year as i32,
            current_month as i32,
            current_day as i32,
        ),
        (target_year as i32, target_month as i32, target_day as i32),
    );

    if target_year - current_year > 1000 {
        println!("gg");
    } else if target_year - current_year == 1000
        && target_month >= current_month
        && target_day >= current_day
    {
        println!("gg");
    } else {
        println!("D-{}", d_day);
    }
}
