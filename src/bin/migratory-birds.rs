#![allow(non_snake_case)]

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    // As per question for all i 1<= arr[i] <= 5
    let mut frequency_counts = BTreeMap::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]);

    let mut most_frequent_bird = 1;
    let mut highest_frequency = 0;

    for bird_id in arr {
        *frequency_counts.get_mut(bird_id).unwrap() += 1;
    }

    // Uses assumption that BTreeMap returns keys in sorted order
    for (bird_id, freq) in frequency_counts {
        if freq > highest_frequency {
            // Only update if higher, we don't care about equal as we want the lowest id in that case
            most_frequent_bird = bird_id;
            highest_frequency = freq;
        }
    }

    most_frequent_bird
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{result}").ok();
}
