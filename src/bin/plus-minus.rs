use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let mut pos = 0;
    let mut zero = 0;
    let mut neg = 0;
    arr.iter().for_each(|x| match x.cmp(&0) {
        std::cmp::Ordering::Less => neg += 1,
        std::cmp::Ordering::Equal => zero += 1,
        std::cmp::Ordering::Greater => pos += 1,
    });
    let n = arr.len() as f64;
    println!("{:.6}", pos as f64 / n);
    println!("{:.6}", neg as f64 / n);
    println!("{:.6}", zero as f64 / n);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
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

    plusMinus(&arr);
}
