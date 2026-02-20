use std::process::Command;

fn agenticenti() -> Command {
    Command::new(env!("CARGO_BIN_EXE_agenticenti"))
}

#[test]
fn test_compose_single_role() {
    let output = agenticenti().args(["compose", "coder"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(!stdout.is_empty());
}

#[test]
fn test_compose_role_with_overlay() {
    let output = agenticenti()
        .args(["compose", "coder", "rust"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("---")); // Has separator between role and overlay
}

#[test]
fn test_compose_with_testing_mode() {
    let output = agenticenti()
        .args(["compose", "tester", "rust", "--testing-mode", "unit"])
        .output()
        .unwrap();
    assert!(output.status.success());
}

#[test]
fn test_compose_unknown_role_fails() {
    let output = agenticenti()
        .args(["compose", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown"));
}

#[test]
fn test_list_roles() {
    let output = agenticenti().args(["list", "--roles"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("coder"));
    assert!(stdout.contains("evangelist"));
    assert!(stdout.contains("github-actions"));
}

#[test]
fn test_list_overlays() {
    let output = agenticenti().args(["list", "--overlays"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("bash"));
    assert!(stdout.contains("docker"));
}

#[test]
fn test_list_testing_modes() {
    let output = agenticenti()
        .args(["list", "--testing-modes"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("unit"));
    assert!(stdout.contains("e2e"));
}

#[test]
fn test_list_all_default() {
    let output = agenticenti().args(["list"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Roles:"));
    assert!(stdout.contains("Overlays:"));
    assert!(stdout.contains("Testing Modes:"));
}
