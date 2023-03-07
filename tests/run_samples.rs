use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    process::Command,
};

fn get_problem_settings() -> ProblemSettings {
    // Change problem name here <----------------------->

    ProblemSettings {
        problem_name: "countingsort1".to_string(),
        // eval_type: EvalType::Stdout,
        eval_type: EvalType::File(Default::default()),
    }
}

#[test]
fn sample_test_cases() -> Result<(), Box<dyn std::error::Error>> {
    let ProblemSettings {
        problem_name,
        eval_type,
    } = get_problem_settings();

    let problem_samples_folder_name = problem_name.clone() + "-testcases"; // Add on default extension found on download

    let samples_folder: PathBuf = ["tests", "samples", &problem_samples_folder_name]
        .iter()
        .collect(); // Create path to samples in OS agnostic way
    let folder_input = samples_folder.join("input");
    let folder_output = samples_folder.join("output");

    println!("Input folder is: {}", folder_input.to_string_lossy());
    for file in fs::read_dir(&folder_input).map_err(|e| {
        format!(
            "Failed to open input folder: {}. Error: {e}",
            folder_input.to_string_lossy()
        )
    })? {
        // Clear output from last run if necessary
        if let EvalType::File(settings) = &eval_type {
            let path = Path::new(&settings.file_name);
            match path.try_exists() {
                Ok(true) => fs::remove_file(path).map_err(|e| {
                    format!(
                        "An error occurred while removing {}. Error: {e}",
                        path.display()
                    )
                })?,
                Ok(false) => (), // Do nothing there is no file to delete
                Err(e) => panic!(
                    "Error while check if file exists. Filename: {}. Error: {e}",
                    path.display()
                ),
            }
        }

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
            .replace("\r\n", "\n")
            .to_owned();

        // Log which input for ease of reference on error
        println!(
            "Going to test on files: Input / Output - {} / {}",
            input_path.file_name().unwrap().to_string_lossy(),
            output_filename
        );

        // Trim needed because sample output does not have a trailing line ending
        let mut cmd = Command::cargo_bin(&problem_name)?;
        cmd.stdin(File::open(file.path())?);
        match &eval_type {
            EvalType::Stdout => {
                cmd.assert()
                    .stdout(predicate::str::diff(expected_output).trim().normalize());
            }
            EvalType::File(settings) => {
                cmd.env(&settings.env_var_name, &settings.file_name);
                cmd.assert().success();
                let actual = fs::read_to_string(&settings.file_name)
                    .map_err(|e| {
                        format!(
                            "Expected output of executable at {}. Error: {e}",
                            &settings.file_name
                        )
                    })
                    .expect("Failed to get actual output")
                    .trim()
                    .to_owned();
                assert_eq!(actual, expected_output);
            }
        }
    }
    Ok(())
}

fn get_scrap_folder() -> PathBuf {
    ["tests", "scrap"].iter().collect()
}

#[allow(dead_code)]
enum EvalType {
    Stdout,
    File(TestingFile),
}
struct TestingFile {
    env_var_name: String,
    file_name: String,
}

impl Default for TestingFile {
    fn default() -> Self {
        Self {
            env_var_name: "OUTPUT_PATH".to_string(),
            file_name: get_scrap_folder()
                .join("output_file.txt")
                .to_string_lossy()
                .to_string(),
        }
    }
}

struct ProblemSettings {
    problem_name: String,
    eval_type: EvalType,
}
