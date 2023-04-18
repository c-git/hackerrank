use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sansaXor' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn sansa_xor(arr: &[i32]) -> i32 {
    let mut result = 0;
    for subarray_len in 1..=arr.len() {
        for start in 0..=arr.len() - subarray_len {
            result ^= xor(&arr[start..subarray_len + start]);
        }
    }
    result
}

fn xor(arr: &[i32]) -> i32 {
    debug_assert!(!arr.is_empty());
    let mut result = arr[0];
    for val in arr.iter().skip(1) {
        result ^= val;
    }
    result
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
        let _n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let arr: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = sansa_xor(&arr);

        writeln!(&mut fptr, "{result}").ok();
    }
}
