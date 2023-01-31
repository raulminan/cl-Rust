use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Tests if echor dies and shows help if no input arguments
/// are given
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure() 
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt");
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt");
    Ok(())
}

#[test]
fn hello3() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello3.txt");
    Ok(())
}

#[test]
fn hello4() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello4.txt");
    Ok(())
}
