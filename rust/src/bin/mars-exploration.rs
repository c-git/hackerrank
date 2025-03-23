use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'marsExploration' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */
#[allow(non_snake_case)]
fn marsExploration(s: &str) -> i32 {
    let mut result = 0;
    let mut read_pos = ExpectedCharInPosition::First;
    for c in s.chars() {
        if c != read_pos.expected_char() {
            result += 1;
        }
        read_pos = read_pos.next();
    }
    result
}

enum ExpectedCharInPosition {
    First,
    Second,
    Third,
}

impl ExpectedCharInPosition {
    fn expected_char(&self) -> char {
        match self {
            Self::First => 'S',
            Self::Second => 'O',
            Self::Third => 'S',
        }
    }

    fn next(&self) -> ExpectedCharInPosition {
        match self {
            Self::First => Self::Second,
            Self::Second => Self::Third,
            Self::Third => Self::First,
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = marsExploration(&s);

    writeln!(&mut fptr, "{result}").ok();
}
