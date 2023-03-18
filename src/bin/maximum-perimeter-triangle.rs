#![allow(non_snake_case)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'maximumPerimeterTriangle' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY sticks as parameter.
 */

fn maximumPerimeterTriangle(sticks: &[i32]) -> Vec<i32> {
    let not_found = vec![-1];
    let mut sticks = sticks.to_owned();
    sticks.sort();
    let mut iter = sticks.iter().rev();
    let mut a = if let Some(&val) = iter.next() {
        val
    } else {
        return not_found;
    };
    let mut b = if let Some(&val) = iter.next() {
        val
    } else {
        return not_found;
    };
    for &c in iter {
        // See if the three sides form a triangle (if found this would be the largest possible because input was sorted)
        if a < b + c {
            return vec![c, b, a];
        } else {
            // Push out a and include c in checking
            a = b;
            b = c;
        }
    }
    not_found
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

    let sticks: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = maximumPerimeterTriangle(&sticks);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
