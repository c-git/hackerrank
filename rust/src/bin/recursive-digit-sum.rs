use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'superDigit' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING n
 *  2. INTEGER k
 */

fn super_digit(n: &str, k: i32) -> i32 {
    // Wasn't able to understand the editorial but looking at someone else's solution I discovered that the mistake was adding k times instead of multiplying by k
    // Originally made a mistake in check if k * digits_sum was the same as adding each of the times.

    if n.len() <= 1 {
        return n.parse().unwrap();
    }
    let mut digits_sum: usize = 0;
    digits_sum += n
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum::<usize>();
    digits_sum *= k as usize;

    while digits_sum > 9 {
        let mut value = digits_sum;
        digits_sum = 0;
        while value > 0 {
            digits_sum += value % 10;
            value /= 10;
        }
    }

    digits_sum as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = &first_multiple_input[0];

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let result = super_digit(n, k);

    writeln!(&mut fptr, "{}", result).ok();
}
