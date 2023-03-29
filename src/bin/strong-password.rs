use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'minimumNumber' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. STRING password
 */

fn minimum_number(n: i32, password: &str) -> i32 {
    // Return the minimum number of characters to make the password strong
    debug_assert!(n == password.len() as i32);
    let min_password_length = 6;
    let mut has_digit = false;
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_special = false;

    let mut special_characters = HashSet::new();
    for c in "!@#$%^&*()-+".chars() {
        special_characters.insert(c);
    }

    for c in password.chars() {
        match c {
            '0'..='9' => has_digit = true,
            'a'..='z' => has_lower = true,
            'A'..='Z' => has_upper = true,
            _ => {
                debug_assert!(special_characters.contains(&c)); // Should be true based on constrains in question that it must be in one of the sets
                has_special = true
            }
        }
    }

    let result = if has_digit { 0 } else { 1 }
        + if has_lower { 0 } else { 1 }
        + if has_upper { 0 } else { 1 }
        + if has_special { 0 } else { 1 };

    result.max(min_password_length - n)
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
        .parse::<i32>()
        .unwrap();

    let password = stdin_iterator.next().unwrap().unwrap();

    let answer = minimum_number(n, &password);

    writeln!(&mut fptr, "{answer}").ok();
}
