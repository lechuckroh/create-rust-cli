use std::fs;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const EXECUTABLE: &str = "exif-rename";

fn run(args: &[&str], expected_lines: &[&str]) -> TestResult {
    let expected = expected_lines.join("\n") + "\n";
    Command::cargo_bin(EXECUTABLE)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_lines: &[&str]) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = expected_lines.join("\n") + "\n";
    Command::cargo_bin(EXECUTABLE)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// ------------------------------------------------------
#[test]
fn jpg_with_exif() -> TestResult {
    run(&["--exif", "tests/inputs/exif-jpg.txt", "--pattern", "{y}{m}{D}_{t}_{T2}_{r}.{e}"],
        &["230908_185654_iPhone 14_2533.JPG"])
}

#[test]
fn stdin_exif() -> TestResult {
    run_stdin("tests/inputs/exif-jpg.txt",
              &["--pattern", "{y}{m}{D}_{t}_{T2}_{r}.{e}"],
              &["230908_185654_iPhone 14_2533.JPG"])
}
