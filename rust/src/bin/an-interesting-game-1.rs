use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gamingArray' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn gaming_array(arr: &[i32]) -> String {
    let mut rounds_played = 0;

    // Create list of tuples with the values and their positions
    let mut values_and_pos: Vec<(i32, usize)> =
        arr.iter().enumerate().map(|(i, &val)| (val, i)).collect();

    // Sort to be able to get largest values
    let mut max_index_remaining = arr.len() - 1;
    values_and_pos.sort();

    // Step through collection and see how many moves before it gets empty
    for &(_val, i) in values_and_pos.iter().rev() {
        if i <= max_index_remaining {
            rounds_played += 1;
            max_index_remaining = i;
        }
    }

    if rounds_played % 2 == 0 {
        "ANDY".to_string()
    } else {
        "BOB".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let g = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..g {
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

        let result = gaming_array(&arr);

        writeln!(&mut fptr, "{result}").ok();
    }
}
