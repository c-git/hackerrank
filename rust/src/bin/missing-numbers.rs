use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'missingNumbers' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY arr
 *  2. INTEGER_ARRAY brr
 */

fn missing_numbers(arr: &[i32], brr: &[i32]) -> Vec<i32> {
    let mut missing: HashMap<i32, i32> = HashMap::new();

    let mut arr_iter = arr.iter();
    let mut brr_iter = brr.iter();
    let mut arr_element = arr_iter.next();
    let mut brr_element = brr_iter.next();
    while arr_element.is_some() || brr_element.is_some() {
        if let Some(element) = arr_element {
            *missing.entry(*element).or_default() -= 1;
        }

        if let Some(element) = brr_element {
            *missing.entry(*element).or_default() += 1;
        }
        arr_element = arr_iter.next();
        brr_element = brr_iter.next();
    }

    let mut missing: Vec<i32> = missing
        .iter()
        .filter_map(|(key, value)| if *value != 0 { Some(*key) } else { None })
        .collect();
    missing.sort();
    missing
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

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _m = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let brr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = missing_numbers(&arr, &brr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
