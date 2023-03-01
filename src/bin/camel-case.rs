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
    let (operation, identifier_type, words) = split_input(line);
    match operation {
        OperationType::Split => split_words(&words, &identifier_type),
        OperationType::Combine => combine_words(&words, &identifier_type),
    }
}

fn combine_words(words: &str, identifier_type: &IdentifierType) {
    let mut result = "".to_owned();
    let mut is_first_word = true; // Track if it's the fist word as it will require special treatment

    for word in words.split(' ') {
        assert!(
            word == word.to_lowercase(),
            "Expected input of only lower case characters"
        );
        if is_first_word {
            if identifier_type == &IdentifierType::Class {
                result += &capitalize_first_char(word);
            } else {
                result += word;
            }
            is_first_word = false;
        } else {
            result += &capitalize_first_char(word);
        }
    }
    if identifier_type == &IdentifierType::Method {
        result += "()"
    }
    println!("{result}");
}

fn split_words(words: &str, identifier_type: &IdentifierType) {
    assert!(!words.is_empty(), "Expected a word but got an empty string");
    let mut result: Vec<String> = vec![];
    let mut start_index = 0;
    for (i, c) in words.char_indices() {
        if i == 0 {
            continue;
        }
        if c.is_uppercase() {
            result.push(words[start_index..i].to_lowercase());
            start_index = i;
        }
    }

    // Ensure last word is captured
    let end_index = words.len()
        - if identifier_type == &IdentifierType::Method {
            2
        } else {
            0
        };
    result.push(words[start_index..end_index].to_lowercase());

    println!("{}", result.join(" "));
}

fn split_input(line: &str) -> (OperationType, IdentifierType, String) {
    let parts: Vec<_> = line.split(';').collect();
    assert!(
        parts.len() == 3,
        "Expected exactly 3 parts to the input but got {}",
        parts.len()
    );
    (
        OperationType::new(parts[0]),
        IdentifierType::new(parts[1]),
        parts[2].to_string(),
    )
}

#[derive(PartialEq, Eq)]
enum OperationType {
    Split,
    Combine,
}
impl OperationType {
    pub(crate) fn new(value: &str) -> Self {
        match value {
            "S" => Self::Split,
            "C" => Self::Combine,
            _ => unreachable!(
                "Assumed based on question that only S & C were valid for Operation Type"
            ),
        }
    }
}

#[derive(PartialEq, Eq)]
enum IdentifierType {
    Method,
    Class,
    Variable,
}
impl IdentifierType {
    pub(crate) fn new(value: &str) -> Self {
        match value {
            "M" => Self::Method,
            "C" => Self::Class,
            "V" => Self::Variable,
            _ => unreachable!(
                "Assumed based on question that only M, C and V were valid for Identifier Type"
            ),
        }
    }
}

fn capitalize_first_char(s: &str) -> String {
    // Assumes input is only single byte unicode characters
    let mut result = String::with_capacity(s.len());
    for (i, c) in s.char_indices() {
        result.push(if i == 0 { c.to_ascii_uppercase() } else { c });
    }
    result
}
