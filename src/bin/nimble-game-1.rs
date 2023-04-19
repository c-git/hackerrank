use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'nimbleGame' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY s as parameter.
 */

fn nimble_game(s: &[i32]) -> String {
    // Count the number of squares other than 0 that have a odd number of coins.
    // All other squares allow for mirror play.
    // If there is an odd number of squares with an odd total then first wins because second will end up with no move else second wins
    let odd_sum: usize = s
        .iter()
        .skip(1)
        .map(|&x| if x % 2 == 0 { 0 } else { 1 })
        .sum();
    if odd_sum % 2 == 1 {
        "First".to_string()
    } else {
        "Second".to_string()
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
        let _n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let s: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = nimble_game(&s);

        writeln!(&mut fptr, "{result}").ok();
    }
}
