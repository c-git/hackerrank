use std::io::{self, BufRead, Write};

/*
 * Complete the 'countSort' function below.
 *
 * The function accepts 2D_STRING_ARRAY arr as parameter.
 */

fn count_sort(arr: &[Vec<String>]) {
    // Create 100 slots to store values based on constraints given in question
    let mut sorted_values = vec![vec![]; 100];

    let dash = "-".to_string();
    for (i, pair) in arr.iter().enumerate() {
        let x: usize = pair[0].parse().expect("Should be int based on question");
        let s = if i < arr.len() / 2 { &dash } else { &pair[1] };
        sorted_values[x].push(s);
    }

    for values in sorted_values {
        for value in values {
            print!("{value} ");
        }
    }
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

    let mut arr: Vec<Vec<String>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(2_usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
    }

    count_sort(&arr);
}
