use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthday' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY s
 *  2. INTEGER d
 *  3. INTEGER m
 */

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut result = 0;
    let m = m as usize;
    let mut sum: i32 = s[0..m].iter().sum();
    if sum == d {
        result += 1;
    }
    for i in m..s.len() {
        let old_pos = i - m;
        sum -= s[old_pos];
        sum += s[i];
        if sum == d {
            result += 1;
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let s: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = birthday(&s, d, m);

    writeln!(&mut fptr, "{result}").ok();
}
