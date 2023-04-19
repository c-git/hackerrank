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
    // I was wrong, there was a flaw in my reasoning and I don't quite know what it was and the editorial didn't
    // quite clear it up but reimplementing their explanation here.
    // Do xor sum unless all entries and first wins if sum is > 0,
    // unless 1 is the only value in the input then give result based on cardinality of input. If even then first wins else they lose.
    // Will have to come back to this it's not clear why this is the solution

    if s.iter().all(|&x| x == 1) {
        // All entries are 1
        if s.len() % 2 == 0 {
            "First"
        } else {
            "Second"
        }
    } else {
        // Use xor sum approach
        let mut xor_sum = 0;
        for &val in s {
            debug_assert!(val > 0);
            xor_sum ^= val;
        }

        if xor_sum > 0 {
            "First"
        } else {
            "Second"
        }
    }
    .to_string()
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
