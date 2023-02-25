use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::{
    fs::{self, File},
    path::PathBuf,
    process::Command,
};

// Change problem name here <----------------------->
const PROBLEM_NAME: &str = "plus-minus";

#[test]
fn sample_test_cases() -> Result<(), Box<dyn std::error::Error>> {
    let problem_samples_folder_name = PROBLEM_NAME.to_owned() + "-testcases"; // Add on default extension found on download

    let samples_folder: PathBuf = ["tests", "samples", &problem_samples_folder_name]
        .iter()
        .collect(); // Create path to samples in OS agnostic way
    let folder_input = samples_folder.join("input");
    let folder_output = samples_folder.join("output");

    for file in fs::read_dir(folder_input)? {
        let file = file?;
        let input_path = file.path();
        let output_filename = input_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace("input", "output");
        let output_path = folder_output.join(&output_filename);
        let output_path_as_str = output_path.to_string_lossy();
        let expected_output = fs::read_to_string(&output_path)
            .map_err(|e| format!("Failed to load output: {output_path_as_str} Error:{e}"))?
            .trim()
            .to_owned();

        // Log which input for ease of reference on error
        println!(
            "Going to test on files: Input / Output - {} / {}",
            input_path.file_name().unwrap().to_string_lossy(),
            output_filename
        );

        // Trim needed because sample output does not have a trailing line ending
        Command::cargo_bin(PROBLEM_NAME)?
            .stdin(File::open(file.path())?)
            .assert()
            .stdout(predicate::str::diff(expected_output).trim());
    }
    Ok(())
}
