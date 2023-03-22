use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pickingNumbers' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

#[allow(non_snake_case)]
fn pickingNumbers(a: &[i32]) -> i32 {
    debug_assert!(a.len() >= 2); // Taken from constraints on the question
    let mut table: Vec<i32> = Vec::with_capacity(a.len());
    let mut longest = 0;
    for outer in 0..a.len() {
        // Create a spot for the current value
        table.push(1); // Will always be at least this digit long

        // Store value in a variable to make code easier to read and avoid the double memory ref for each read in the loop below
        let curr_val = a[outer];
        for inner in 0..outer {
            if (a[inner] - curr_val).abs() <= 1 {
                let candidate_length = table[inner] + 1;
                if candidate_length > table[outer] {
                    table[outer] = candidate_length;
                }
            }
        }

        // Update longest if this one is longer
        if table[outer] > longest {
            longest = table[outer];
        }
    }

    longest
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

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pickingNumbers(&a);

    writeln!(&mut fptr, "{result}").ok();
}
