#![allow(non_snake_case)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'twoArrays' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY A
 *  3. INTEGER_ARRAY B
 */

fn twoArrays(k: i32, A: &[i32], B: &[i32]) -> String {
    // General idea is to assume that all values in B less than the current position in A have already been "used"
    // Therefore find the smallest remaining value in B that gives a sum greater than k
    // If none can be found we know that a different permutation would still not have had an available value because
    // we only used the smallest value that worked in B. Small optimization is to stop the search if the value is equal to k
    // because any smaller value would not work. Then move this now used value to the current position in B.

    let false_value = "NO";
    let true_value = "YES";

    let mut B = B.to_owned();
    for (i, a) in A.iter().enumerate() {
        // LI: All values in B[..i] are not able to be use anymore, if a solution exists for sum(A[i], b) >= k then b must be in B[i..]

        // Find smallest value in B[i:] that meets the requirements
        let b_ind = find_smallest_valid(k, a, &B[i..]);
        if let Some(b_ind) = b_ind {
            B.swap(i, b_ind);
        } else {
            return false_value.to_string();
        }
    }
    true_value.to_string()
}

fn find_smallest_valid(k: i32, a: &i32, B: &[i32]) -> Option<usize> {
    let mut ind_smallest = None;
    let mut smallest = &i32::MAX;
    for (i, b) in B.iter().enumerate() {
        let sum = b + a;
        if sum == k {
            return Some(i);
        } else if sum > k && smallest > b {
            ind_smallest = Some(i);
            smallest = b;
        }
    }
    ind_smallest
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let A: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let B: Vec<i32> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = twoArrays(k, &A, &B);

        writeln!(&mut fptr, "{result}").ok();
    }
}
