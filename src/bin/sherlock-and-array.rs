use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'balancedSums' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn balanced_sums(arr: &[i32]) -> String {
    // i32 is big enough for the sum because arr[i] <= 2e4 and n <= 1e5
    // giving a max value of 2e9 which is a little less than i32::MAX
    let mut left_sum = 0;
    let mut right_sum: i32 = arr.iter().sum();
    for val in arr {
        right_sum -= val;
        if left_sum == right_sum {
            return "YES".to_string();
        }
        left_sum += val;
    }
    "NO".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    #[allow(non_snake_case)]
    let T = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..T {
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

        let result = balanced_sums(&arr);

        writeln!(&mut fptr, "{result}").ok();
    }
}
