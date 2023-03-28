use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'closestNumbers' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
#[allow(non_snake_case)]
fn closestNumbers(arr: &[i32]) -> Vec<i32> {
    debug_assert!(arr.len() >= 2, "As per constraint in question");

    let mut result = vec![];

    let mut arr = arr.to_owned();
    arr.sort();
    let mut iter = arr.iter();

    let mut min_diff = i32::MAX;
    let mut last_num = *iter.next().expect("Min length is 2 as per constraint");
    for &num in iter {
        let diff = num - last_num; // Always positive because array is sorted so no need for abs
        if diff < min_diff {
            min_diff = diff;
            result.clear();
        }
        if min_diff == diff {
            result.push(last_num);
            result.push(num);
        }
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

    let result = closestNumbers(&arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
