use std::collections::BTreeMap;
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
    // After looking at the editorial I understand now that all the values in the subarray must be within 1 abs diff of each other not each value 1 abs from the one next to it
    let mut frequencies = BTreeMap::new();
    for val in a {
        if frequencies.contains_key(val) {
            *frequencies.get_mut(val).unwrap() += 1;
        } else {
            frequencies.insert(val, 1);
        }
    }
    let (last_key, last_val) = frequencies
        .first_key_value()
        .expect("Assuming at min 2 values based on constraints");
    let (mut last_key, mut last_val) = (**last_key, *last_val); // Get copies for values instead
    let mut largest_val = last_val;
    for (&curr_key, curr_val) in frequencies {
        if curr_key - last_key == 1 {
            largest_val = largest_val.max(curr_val + last_val)
        } else {
            largest_val = largest_val.max(curr_val);
        }
        last_key = curr_key;
        last_val = curr_val;
    }
    largest_val
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
