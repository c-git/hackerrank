use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */
#[allow(non_snake_case)]
fn countingValleys(_steps: i32, path: &str) -> i32 {
    let mut result = 0;
    let mut height = 0;
    for c in path.chars() {
        match c {
            'D' => {
                if height == 0 {
                    result += 1;
                }
                height -= 1;
            }
            'U' => height += 1,
            _ => unreachable!("Based on question only D and U are the expected values in path"),
        }
    }
    assert_eq!(height, 0);
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let steps = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let path = stdin_iterator.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);

    writeln!(&mut fptr, "{result}").ok();
}
