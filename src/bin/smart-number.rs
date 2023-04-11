// Converted from python version

use std::io::{self, BufRead};

fn is_smart_number(num: i32) -> bool {
    let val = f32::sqrt(num as f32) as i32;

    #[allow(clippy::needless_bool)]
    if num / val == 1 {
        true
    } else {
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();

    for _ in 0..n {
        let num = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let ans = is_smart_number(num);
        if ans {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
