use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

fn forming_magic_square(s: &[Vec<i32>]) -> i32 {
    // First attempt brute force to find possible solutions and compare
    let possible_solutions = [
        [vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]],
        [vec![2, 9, 4], vec![7, 5, 3], vec![6, 1, 8]],
        [vec![4, 3, 8], vec![9, 5, 1], vec![2, 7, 6]],
        [vec![4, 9, 2], vec![3, 5, 7], vec![8, 1, 6]],
        [vec![6, 1, 8], vec![7, 5, 3], vec![2, 9, 4]],
        [vec![6, 7, 2], vec![1, 5, 9], vec![8, 3, 4]],
        [vec![8, 1, 6], vec![3, 5, 7], vec![4, 9, 2]],
        [vec![8, 3, 4], vec![1, 5, 9], vec![6, 7, 2]],
    ];

    let mut smallest = i32::MAX;

    for possible_solution in possible_solutions {
        let diff = diff_from_solution(s, &possible_solution);
        if diff == 0 {
            return 0;
        } else if diff < smallest {
            smallest = diff;
        }
    }

    smallest
}

fn diff_from_solution(candidate: &[Vec<i32>], solution: &[Vec<i32>]) -> i32 {
    let mut result = 0;
    for (candidate_row, solution_row) in candidate.iter().zip(solution) {
        for (c, s) in candidate_row.iter().zip(solution_row) {
            result += (c - s).abs();
        }
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = forming_magic_square(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

#[cfg(test)]
mod tests {
    //! Code used during reasoning about the problem
    use itertools::Itertools;

    #[test]
    fn heaps_algo() {
        let mut count_found = 0;
        let mut count_checked = 0;
        let square = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for perm in square.iter().permutations(square.len()) {
            count_checked += 1;
            if is_magic(&[&perm[0..3], &perm[3..6], &perm[6..9]]) {
                count_found += 1;
                if count_found < 50 {
                    println!("{:?}", perm);
                }
            }
        }
        println!("Found {count_found} solutions of {count_checked} checked");

        panic!("Need this to fail for it to print")
    }

    fn is_magic(s: &[&[&i32]; 3]) -> bool {
        // Assumes 3x3

        // Diagonal 1
        let magic_num = (0..3).map(|i| s[i][i]).sum::<i32>();

        // Diagonal 2
        if (0..3).map(|i| s[2 - i][i]).sum::<i32>() != magic_num {
            return false;
        }

        // Horizontal totals
        for element in s {
            if element.iter().map(|&&x| x).sum::<i32>() != magic_num {
                return false;
            }
        }

        // Vertical totals
        for i in 0..3 {
            if s[0][i] + s[1][i] + s[2][i] != magic_num {
                return false;
            }
        }

        true
    }
}
