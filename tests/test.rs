use std::process::Command;

#[test]
fn normal_run() {
    let output = Command::new(env!("CARGO_BIN_EXE_herobi"))
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(!stdout.is_empty());
}

#[test]
fn category_option() {
    let output = Command::new(env!("CARGO_BIN_EXE_herobi"))
        .arg("-c")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Directories"));
}

#[test]
fn size_option() {
    let output = Command::new(env!("CARGO_BIN_EXE_herobi"))
        .arg("-s")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Directory Size"));
}

#[test]
fn summary_option() {
    let output = Command::new(env!("CARGO_BIN_EXE_herobi"))
        .arg("-m")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Summary"));
}

#[test]
fn all_options() {
    let output = Command::new(env!("CARGO_BIN_EXE_herobi"))
        .args(["-c", "-s", "-m"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Directories"));
    assert!(stdout.contains("Directory Size"));
    assert!(stdout.contains("Summary"));
}

#[test]
fn completion_option() {
    let output = Command::new(env!("CARGO_BIN_EXE_herobi"))
        .arg("--completions")
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Completion files were generated"));
}