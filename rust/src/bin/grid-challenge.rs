use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'gridChallenge' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING_ARRAY grid as parameter.
 */

fn grid_challenge(grid: &[String]) -> String {
    // Convert input into vec
    let mut input: Vec<Vec<char>> = vec![];
    for row in grid {
        let mut row: Vec<char> = row.chars().collect();
        row.sort();
        input.push(row);
    }

    // Check if all columns are sorted
    for col_index in 0..input[0].len() {
        let mut last_char = input[0][col_index];
        for row in input.iter().skip(1) {
            if row[col_index] < last_char {
                return "NO".to_string();
            } else {
                last_char = row[col_index];
            }
        }
    }

    "YES".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..t {
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let mut grid: Vec<String> = Vec::with_capacity(n as usize);

        for _ in 0..n {
            let grid_item = stdin_iterator.next().unwrap().unwrap();
            grid.push(grid_item);
        }

        let result = grid_challenge(&grid);

        writeln!(&mut fptr, "{result}").ok();
    }
}
