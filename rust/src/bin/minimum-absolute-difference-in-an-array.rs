use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'minimumAbsoluteDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn minimum_absolute_difference(mut arr: Vec<i32>) -> i32 {
    debug_assert!(arr.len() >= 2);
    let mut result = i32::MAX;
    arr.sort();
    let mut iter = arr.iter();
    let mut last_num = iter.next().expect("Min length of 2 based on question");

    for num in iter {
        let diff = num - last_num;
        result = result.min(diff);
        last_num = num;
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

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = minimum_absolute_difference(arr);

    writeln!(&mut fptr, "{result}").ok();
}
