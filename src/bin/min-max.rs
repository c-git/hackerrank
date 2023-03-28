use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'maxMin' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY arr
 */

fn max_min(k: i32, mut arr: Vec<i32>) -> i32 {
    let k = k as usize;
    arr.sort();
    let mut result = i32::MAX;
    for i in 0..=arr.len() - k {
        result = result.min(arr[i + k - 1] - arr[i]);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let k = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let arr_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        arr.push(arr_item);
    }

    let result = max_min(k, arr);

    writeln!(&mut fptr, "{result}").ok();
}
