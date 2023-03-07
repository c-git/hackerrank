use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'flippingMatrix' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY matrix as parameter.
 */
#[allow(non_snake_case)]
fn flippingMatrix(matrix: &[Vec<i32>]) -> i32 {
    // After reviewing editorial (Didn't clock that each cell could be independently set before reading the editorial)
    let n2 = matrix.len();
    let n = matrix.len() / 2;
    let mut result = 0;

    // Each cell has 4 possible positions so pick the max from each position
    for row in 0..n {
        for col in 0..n {
            result += [
                matrix[row][col],
                matrix[n2 - row - 1][col],
                matrix[n2 - row - 1][n2 - col - 1],
                matrix[row][n2 - col - 1],
            ]
            .iter()
            .fold(i32::MIN, |a, b| a.max(*b));
        }
    }
    result
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
        let n = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();

        let mut matrix: Vec<Vec<i32>> = Vec::with_capacity((2 * n) as usize);

        for i in 0..(2 * n) as usize {
            matrix.push(Vec::with_capacity((2 * n) as usize));

            matrix[i] = stdin_iterator
                .next()
                .unwrap()
                .unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let result = flippingMatrix(&matrix);

        writeln!(&mut fptr, "{result}").ok();
    }
}
