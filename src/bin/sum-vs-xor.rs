use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sumXor' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER n as parameter.
 */

fn sum_xor(n: i64) -> i64 {
    //! A number is the same as itself added with another number when they share no bits in common
    //! Thus we need to find out how many numbers we can make using the zero spaces in the number
    //! If there are no spaces then we can still use 0 otherwise we need we need to count how many
    //! values we can get. That is the same as taking none, n choose 1 and so on. This is the same
    //! number of elements in the power set which is 2^n. Another way to think of it is you need to
    //! consider the values attainable from having each bit either on or off, which is 2^n values.

    let num_zeros = n.count_zeros() - n.leading_zeros();
    2_i64.pow(num_zeros)
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
        .parse::<i64>()
        .unwrap();

    let result = sum_xor(n);

    writeln!(&mut fptr, "{}", result).ok();
}

#[test]
fn bute_force() {
    let mut total = 0;
    let n = 90;
    for x in 0..n {
        if n + x == x ^ n {
            total += 1;
        }
    }
    assert_eq!(total, 8);
}
