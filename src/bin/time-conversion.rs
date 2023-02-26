use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    assert!(
        s.chars().count() == 10,
        "Expected exactly 10 character but got {}",
        s.len()
    );
    let hours = get_hours(s);
    let min_and_sec: String = s.chars().skip(2).take(6).collect();
    let hours = if hours == 12 {
        if is_am(s) {
            "00".to_owned()
        } else {
            hours.to_string()
        }
    } else if is_am(s) {
        hours.to_string()
    } else {
        (hours + 12).to_string()
    };

    hours + &min_and_sec
}

fn is_am(s: &str) -> bool {
    s.chars().nth(8) == Some('A')
}

fn get_hours(s: &str) -> u8 {
    let first_two_char: String = s.chars().take(2).collect();
    first_two_char
        .parse()
        .expect("Expected numbers based on question")
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
