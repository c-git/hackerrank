use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'towerBreakers' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 */

fn tower_breakers(n: i32, m: i32) -> i32 {
    // The relationship wasn't obvious to me, read the tutorial and in hindsight I can see how it makes sense based on the ability to always be able to copy
    if m == 1 || n % 2 == 0 {
        2
    } else {
        1
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = tower_breakers(n, m);

        writeln!(&mut fptr, "{result}").ok();
    }
}
