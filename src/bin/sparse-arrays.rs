use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'matchingStrings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY strings
 *  2. STRING_ARRAY queries
 */
#[allow(non_snake_case)]
fn matchingStrings(strings: &[String], queries: &[String]) -> Vec<i32> {
    // Insert queries into a map
    let mut word_counts = HashMap::new();
    for query in queries {
        word_counts.insert(query, 0);
    }

    // Loop through strings and update queries counts
    for s in strings {
        if let Some(count) = word_counts.get_mut(s) {
            *count += 1;
        }
    }

    // Assemble output from counts collected
    let mut result = vec![];
    for query in queries {
        result.push(
            *word_counts
                .get(query)
                .expect("ALL Words should have been in the map"),
        )
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let strings_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut strings: Vec<String> = Vec::with_capacity(strings_count as usize);

    for _ in 0..strings_count {
        let strings_item = stdin_iterator.next().unwrap().unwrap();
        strings.push(strings_item);
    }

    let queries_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    let res = matchingStrings(&strings, &queries);

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
