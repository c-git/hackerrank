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
    let mut is_left_larger = vec![];
    let n2 = matrix.len();
    let n = n2 / 2;
    for row in matrix.iter() {
        let left: i32 = row[0..n].iter().sum();
        let right: i32 = row[n..n2].iter().sum();
        is_left_larger.push(left >= right);
    }

    let mut result = 0;
    for col in 0..n {
        // Calculate value of left
        let mut top = 0;
        for row in 0..n {
            let col = if is_left_larger[row] {
                col
            } else {
                n2 - col - 1
            };
            top += matrix[row][col];
        }

        // Calculate value of right
        let mut bottom = 0;
        for row in n..n2 {
            let col = if is_left_larger[row] {
                col
            } else {
                n2 - col - 1
            };
            bottom += matrix[row][col];
        }
        result += if top > bottom { top } else { bottom };
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
