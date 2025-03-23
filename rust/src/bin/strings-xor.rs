use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn strings_xor(s: &str, t: &str) -> String {
    let mut res = "".to_string();
    for (s_i, t_i) in s.chars().zip(t.chars()) {
        if s_i != t_i && (s_i == '1' || t_i == '0') {
            res += "1";
        } else {
            res += "0";
        }
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap().trim().to_string();
    let t = stdin_iterator.next().unwrap().unwrap().trim().to_string();

    let result = strings_xor(&s, &t);

    writeln!(&mut fptr, "{result}").ok();
}
