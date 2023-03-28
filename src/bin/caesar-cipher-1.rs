use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'caesarCipher' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. INTEGER k
 */

fn caesar_cipher(s: &str, k: u8) -> String {
    let mut result = String::new();
    for c in s.bytes() {
        result.push(match c {
            65..=90 => (c - 65 + k) % 26 + 65,
            97..=122 => (c - 97 + k) % 26 + 97,
            other_val => other_val,
        } as char);
    }
    result
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

    let s = stdin_iterator.next().unwrap().unwrap();

    // Changed to u8 because possible values are 0..=100
    let k = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<u8>()
        .unwrap();

    let result = caesar_cipher(&s, k);

    writeln!(&mut fptr, "{result}").ok();
}
