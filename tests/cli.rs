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

// ------------------------------------------------------
#[test]
fn jpg_with_exif() -> TestResult {
    run(&["--exif", "tests/inputs/exif-jpg.txt", "--pattern", "{y}{m}{D}_{t}_{T2}_{r}.{e}"],
        &["230908_185654_iPhone 14_2533.JPG"])
}
