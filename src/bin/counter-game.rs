use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'counterGame' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts LONG_INTEGER n as parameter.
 */

fn counter_game(mut n: i64) -> String {
    // Based on editorial alternate solution
    n -= 1;
    let mut bits_count = 0;

    while n > 0 {
        bits_count += n % 2;
        n /= 2;
    }

    if bits_count % 2 == 0 {
        "Richard".to_owned()
    } else {
        "Louise".to_owned()
    }
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
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();

        let result = counter_game(n);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
