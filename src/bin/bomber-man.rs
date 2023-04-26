use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'bomberMan' function below.
 *
 * The function is expected to return a STRING_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. STRING_ARRAY grid
 */

enum Cell {
    Empty,
    Bomb(u8),
}

impl Cell {
    fn new_bomb() -> Self {
        Self::Bomb(3)
    }

    fn clear_if_not_exploding_bomb(&mut self) {
        if let Self::Bomb(left) = self {
            if *left > 1 {
                *self = Self::Empty;
            }
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'O' => Self::Bomb(3),
            _ => unreachable!("Constraints are that cells can be a bomb or nothing"),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Empty => '.',
            Cell::Bomb(_) => 'O',
        }
    }
}

fn bomber_man(n: i32, grid: &[String]) -> Vec<String> {
    // Build board state
    let mut board = vec![];
    for row_input in grid {
        let row_board: Vec<Cell> = row_input.chars().map(|x| x.into()).collect();
        board.push(row_board);
    }
    let row_count = board.len();
    let col_count = board[0].len();
    let mut states: Vec<Vec<String>> = vec![];

    // Adjust n to reduce repeated iterations
    let n = if n <= 1 { n } else { (n - 2) % 4 + 2 };

    // Run simulation
    for time_step in 1..=n {
        #[cfg(debug_assertions)]
        print_board(&board, &mut states, time_step);

        let place_bombs = time_step % 2 == 0;
        for row in 0..row_count {
            for col in 0..col_count {
                let cell = &mut board[row][col];
                if place_bombs {
                    // Bombs going to be placed and timers decremented but no explosions allowed
                    match cell {
                        Cell::Empty => {
                            *cell = Cell::new_bomb();
                        }
                        Cell::Bomb(left) => {
                            debug_assert!(*left > 1);
                            *left -= 1;
                        }
                    }
                } else {
                    // Prepare check for explosions
                    if let Cell::Bomb(left) = &mut board[row][col] {
                        if *left == 1 {
                            // Explode
                            board[row][col] = Cell::Empty;
                            if row > 0 {
                                board[row - 1][col].clear_if_not_exploding_bomb();
                            }
                            if row < row_count - 1 {
                                board[row + 1][col].clear_if_not_exploding_bomb();
                            }
                            if col > 0 {
                                board[row][col - 1].clear_if_not_exploding_bomb();
                            }
                            if col < col_count - 1 {
                                board[row][col + 1].clear_if_not_exploding_bomb();
                            }
                        } else {
                            debug_assert!(*left > 1);
                            *left -= 1;
                        }
                    }
                }
            }
        }
    }

    // Convert board state into output
    convert_board_to_strings(&board)
}

fn print_board(board: &[Vec<Cell>], states: &mut Vec<Vec<String>>, time_step: i32) {
    let to_print = convert_board_to_strings(board);

    let state = get_or_insert_state(states, &to_print);

    println!("\n\nTime: {time_step}. State: {state}\n");

    for i in 0..to_print.len() {
        print!("{}", to_print[i]);

        if i != to_print.len() - 1 {
            println!();
        }
    }
}

fn get_or_insert_state(states: &mut Vec<Vec<String>>, to_print: &Vec<String>) -> usize {
    if let Some(i) =
        states
            .iter()
            .enumerate()
            .find_map(|(i, x)| if *x == *to_print { Some(i) } else { None })
    {
        i
    } else {
        states.push(to_print.clone());
        states.len() - 1
    }
}

fn convert_board_to_strings(board: &[Vec<Cell>]) -> Vec<String> {
    let mut result = vec![];
    let col_count = board[0].len();
    for row_board in board {
        let mut row_output: String = String::with_capacity(col_count);
        row_board.iter().for_each(|x| row_output.push(x.into()));
        result.push(row_output);
    }
    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let r = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let _c = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let n = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let mut grid: Vec<String> = Vec::with_capacity(r as usize);

    for _ in 0..r {
        let grid_item = stdin_iterator.next().unwrap().unwrap();
        grid.push(grid_item);
    }

    let result = bomber_man(n, &grid);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
