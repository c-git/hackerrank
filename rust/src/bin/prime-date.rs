use std::io::{self, BufRead};

fn update_leap_year(year: i32, month: &mut [i32]) {
    if year % 400 == 0 {
        month[2] = 29
    } else if year % 100 == 0 {
        month[2] = 28
    } else if year % 4 == 0 {
        month[2] = 29
    } else {
        month[2] = 28
    }
}

fn store_month(month: &mut [i32]) {
    month[1] = 31;
    month[2] = 28;
    month[3] = 31;
    month[4] = 30;
    month[5] = 31;
    month[6] = 30;
    month[7] = 31;
    month[8] = 31;
    month[9] = 30;
    month[10] = 31;
    month[11] = 30;
    month[12] = 31;
}

fn find_prime_dates(
    mut d1: i32,
    mut m1: i32,
    mut y1: i32,
    d2: i32,
    m2: i32,
    y2: i32,
    month: &mut [i32],
) -> i32 {
    store_month(month);
    let mut result = 0;

    loop {
        let mut x = d1;
        x = x * 100 + m1;
        x = x * 10000 + y1;
        if x % 4 == 0 || x % 7 == 0 {
            result += 1
        }
        if d1 == d2 && m1 == m2 && y1 == y2 {
            break;
        }
        update_leap_year(y1, month);
        d1 += 1;
        if d1 > month[m1 as usize] {
            m1 += 1;
            d1 = 1;
            if m1 > 12 {
                y1 += 1;
                m1 = 1
            }
        }
    }
    result
}

fn main() {
    let mut month = vec![];
    for _i in 1..15 {
        month.push(31);
    }

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let date = stdin_iterator.next().unwrap().unwrap();
    let date: Vec<&str> = date.trim().split(&['-', ' ']).collect();

    let d1: i32 = date[0].parse().unwrap();
    let m1: i32 = date[1].parse().unwrap();
    let y1: i32 = date[2].parse().unwrap();
    let d2: i32 = date[3].parse().unwrap();
    let m2: i32 = date[4].parse().unwrap();
    let y2: i32 = date[5].parse().unwrap();

    let result = find_prime_dates(d1, m1, y1, d2, m2, y2, &mut month);
    print!("{result}");
}
