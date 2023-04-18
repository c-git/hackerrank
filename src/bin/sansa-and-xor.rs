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
    // The exponential time algorithm always wasn't going to work but just ran it to ensure it only failed for time to ensure I understood the problem
    // Then I tried a few cases with powers of 2 to see which values contributed to the solution as I noticed the if the number of times the number
    // shows up is even then it will not impact the final result because the same number xor with itself is always 0.
    // So tried some and noticed the pattern
    // | List length | Answer | Contributing Indices | List Values                   |
    // | ----------- | ------ | -------------------- | ----------------------------- |
    // | 1           | 1      | [0]                  | [1]                           |
    // | 2           | 0      | []                   | [1, 2]                        |
    // | 3           | 5      | [0, 2]               | [1, 2, 4]                     |
    // | 4           | 0      | []                   | [1, 2, 4, 8]                  |
    // | 5           | 21     | [0, 2, 4]            | [1, 2, 4, 8, 16]              |
    // | 6           | 0      | []                   | [1, 2, 4, 8, 16, 32]          |
    // | 7           | 85     | [0, 2, 4, 6]         | [1, 2, 4, 8, 16, 32, 64]      |
    // | 8           | 0      | []                   | [1, 2, 4, 8, 16, 32, 64, 128] |
    // Only odd length lists have a non zero result and only even indices are included

    if arr.len() % 2 == 0 {
        return 0;
    }

    let mut result = 0;
    for val in arr.iter().step_by(2) {
        // Take all even index values
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
