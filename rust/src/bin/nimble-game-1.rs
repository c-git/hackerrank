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
    // Understood the reduction to the nim game given in the editorial even thought I don't understand the nim game (but I understood the solution)
    let result: usize = s.iter().enumerate().skip(1).fold(0, |acc, (i, &element)| {
        acc ^ if element % 2 == 0 { 0 } else { i }
    });
    if result > 0 {
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
