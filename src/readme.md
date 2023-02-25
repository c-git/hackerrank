# Hacker Rank Solutions and Testing Framework

I've created this repo to track the problems I've solved and make my testing setup available to others who are still learning rust and want to do the solutions locally to make use of rust-analyzer while learning.

# Where to put problem solutions

A separate binary is created for each problem. They can be found in `src/bin`. To start copy the code as is from Hacker Rank. The name of the file created should match the sample test files downloaded (without the "-testcases" suffix). To run the problem manually use `cargo run --bin <problem_name>`. For more info see <https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=bin#binaries>.

# How to test

- Download the sample test files from Hacker Rank
- Extract the downloaded files (Expected folder name should match problem name, less suffix)
- Update the problem name in `tests/run_samples.rs`. (Look for comment indicating which constant to update near the top of the file)
- Then just use `cargo test` as normal to run the test cases on your solution