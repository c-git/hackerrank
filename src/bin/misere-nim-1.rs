use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'misereNim' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY s as parameter.
 */

fn misere_nim(s: &[i32]) -> String {
    // A column with exactly 1 stone takes exactly 1 move
    // If a column has more than 1 stone then it can be done in either 1 or 2 moves or more but then it can still be finished in 1 or 2 moves
    // For "First" to win there must be an even number of moves played. We can think of the columns as pairing up
    // each column with 1 matches with each other column with 1 unless there is an odd amount then 1 will remain
    // same for the columns with more than 1, they can pair off unless there is an odd amount then it guarantees 2 moves
    // because the other player cannot mirror the move or remove a 2 column if left as 2 or more move column

    let mut result = 0; // First wins if this number is divisible by 2. (0 counts as divisible by 2)

    for &val in s {
        debug_assert!(val > 0, "Value cannot be 0 based on constraints");
        result ^= if val == 0 { 1 } else { 2 };
    }

    if result & 2 == 0 {
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

        let result = misere_nim(&s);

        writeln!(&mut fptr, "{result}").ok();
    }
}
