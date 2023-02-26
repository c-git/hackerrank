#![allow(dead_code)]
#![allow(non_snake_case)]
use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    assert!(
        !arr.is_empty(),
        "Question stated the input would be exactly 5 numbers. Empty slice not expected"
    );
    let mut min_pos = 0;
    let mut max_pos = 0;
    let mut min_val = arr[0];
    let mut max_val = arr[0];
    for (i, &val) in arr.iter().enumerate() {
        if val < min_val {
            min_pos = i;
            min_val = val;
        }
        if val > max_val {
            max_pos = i;
            max_val = val;
        }
    }
    let sum: u64 = arr.iter().map(|&x| x as u64).sum();
    let min_sum = sum - arr[max_pos] as u64;
    let max_sum = sum - arr[min_pos] as u64;
    println!("{min_sum} {max_sum}")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
