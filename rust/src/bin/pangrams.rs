use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pangrams' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn pangrams(s: &str) -> String {
    let full_alphabet_size = 26;
    let s = s.to_lowercase();
    let mut seen = HashSet::new();

    for c in s.chars() {
        if c == ' ' {
            // Skip spaces
            continue;
        }
        seen.insert(c);
        if seen.len() >= full_alphabet_size {
            // All characters seen no point continuing
            break;
        }
    }

    if seen.len() >= full_alphabet_size {
        "pangram".to_string()
    } else {
        "not pangram".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = pangrams(&s);

    writeln!(&mut fptr, "{result}").ok();
}
