use std::io::{self, BufRead};

enum Pretty {
    Pretty(u64),
    NotPretty,
}

fn separate_numbers_helper(s: &str) -> Pretty {
    for i in 1..=s.len() / 2 {
        let first_digit: u64 = s[..i].parse().unwrap();
        if pretty_sequence(first_digit, &s[i..], i) {
            return Pretty::Pretty(first_digit);
        }
    }

    Pretty::NotPretty
}

fn pretty_sequence(first_digit: u64, s: &str, min_str_len: usize) -> bool {
    for min_len in min_str_len..=min_str_len + 1 {
        if s.len() < min_len || &s[0..1] == "0" {
            return false;
        }
        let next_digit: u64 = s[..min_len].parse().unwrap();
        if first_digit < next_digit && next_digit - first_digit == 1 {
            if s.len() == min_len {
                // String complete this sequence works
                return true;
            } else {
                return pretty_sequence(next_digit, &s[min_len..], min_len);
            }
        }
    }

    // Nether min length nor min length plus 1 worked
    false
}
/*
 * Complete the 'separateNumbers' function below.
 *
 * The function accepts STRING s as parameter.
 */
#[allow(non_snake_case)]
fn separateNumbers(s: &str) {
    // High level idea is that once the first digit is determined the rest are fixed
    // So loop over possible values of the first digit and test each

    // Convert to vec of chars to simplify code
    match separate_numbers_helper(s) {
        Pretty::Pretty(first_digit) => println!("YES {first_digit}"),
        Pretty::NotPretty => println!("NO"),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        separateNumbers(&s);
    }
}
