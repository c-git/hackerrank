use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin_iterator = stdin.lock().lines();
    stdin_iterator.into_iter().for_each(|line| {
        process_line(
            &line
                .map_err(|e| format!("Failed to read line with error: {e}"))
                .unwrap(),
        )
    });
}

fn process_line(line: &str) {
    todo!()
}
