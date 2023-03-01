use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lonelyinteger' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn lonelyinteger(a: &[i32]) -> i32 {
    let mut seen = HashSet::new();
    for i in a {
        if seen.contains(i) {
            seen.remove(i);
        } else {
            seen.insert(i);
        }
    }
    assert!(
        seen.len() == 1,
        "Based on odd number of values this should always be true if all others are duplicated"
    );
    if let Some(val) = seen.into_iter().next() {
        return *val;
    }
    unreachable!(
        "Based on the assertion there is one value in seen and it should be returned above"
    );
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

    let result = lonelyinteger(&a);

    writeln!(&mut fptr, "{result}").ok();
}
