use std::process::Command;
use tempfile::TempDir;

fn agenticenti() -> Command {
    Command::new(env!("CARGO_BIN_EXE_agenticenti"))
}

#[test]
fn e2e_compose_coder_rust_produces_valid_sections() {
    let output = agenticenti()
        .args(["compose", "coder", "rust"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Verify structure: role section, separator, overlay section
    let sections: Vec<&str> = stdout.split("\n\n---\n\n").collect();
    assert_eq!(sections.len(), 2, "Expected 2 sections (role + overlay)");
}

#[test]
fn e2e_compose_multi_overlay() {
    let output = agenticenti()
        .args(["compose", "coder", "rust", "docker"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    let sections: Vec<&str> = stdout.split("\n\n---\n\n").collect();
    assert_eq!(sections.len(), 3, "Expected 3 sections (role + 2 overlays)");
}

#[test]
fn e2e_compose_tester_e2e_python_terraform() {
    let output = agenticenti()
        .args([
            "compose",
            "tester",
            "python",
            "terraform",
            "--testing-mode",
            "e2e",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    let sections: Vec<&str> = stdout.split("\n\n---\n\n").collect();
    assert_eq!(
        sections.len(),
        4,
        "Expected 4 sections (role + 2 overlays + testing mode)"
    );
}

#[test]
fn e2e_compose_evangelist_no_overlay() {
    let output = agenticenti()
        .args(["compose", "evangelist"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        !stdout.contains("\n\n---\n\n"),
        "No separator for single section"
    );
}

#[test]
fn e2e_compose_github_actions_docker() {
    let output = agenticenti()
        .args(["compose", "github-actions", "docker"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let sections: Vec<&str> = stdout.split("\n\n---\n\n").collect();
    assert_eq!(sections.len(), 2);
}

#[test]
fn e2e_override_replaces_embedded() {
    let tmp = TempDir::new().unwrap();
    let roles_dir = tmp.path().join("roles");
    std::fs::create_dir_all(&roles_dir).unwrap();
    std::fs::write(
        roles_dir.join("coder.md"),
        "# Role: Custom Coder\n\nThis is my custom coder prompt.",
    )
    .unwrap();

    let output = agenticenti()
        .args([
            "--config-dir",
            tmp.path().to_str().unwrap(),
            "compose",
            "coder",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Custom Coder"));
}

#[test]
fn e2e_user_only_role_from_config_dir() {
    let tmp = TempDir::new().unwrap();
    let roles_dir = tmp.path().join("roles");
    std::fs::create_dir_all(&roles_dir).unwrap();
    std::fs::write(
        roles_dir.join("my-custom-role.md"),
        "# Role: My Custom Role\n\nCustom content here.",
    )
    .unwrap();

    // Should be able to compose with user-only role
    let output = agenticenti()
        .args([
            "--config-dir",
            tmp.path().to_str().unwrap(),
            "compose",
            "my-custom-role",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("My Custom Role"));

    // Should appear in list
    let list_output = agenticenti()
        .args([
            "--config-dir",
            tmp.path().to_str().unwrap(),
            "list",
            "--roles",
        ])
        .output()
        .unwrap();

    let list_stdout = String::from_utf8(list_output.stdout).unwrap();
    assert!(list_stdout.contains("my-custom-role"));
    assert!(list_stdout.contains("(user)"));
}

#[test]
fn e2e_list_shows_all_categories() {
    let output = agenticenti().args(["list"]).output().unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Roles:"));
    assert!(stdout.contains("Overlays:"));
    assert!(stdout.contains("Testing Modes:"));
}

#[test]
fn e2e_unknown_role_exits_with_error() {
    let output = agenticenti()
        .args(["compose", "nonexistent-role"])
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown") || stderr.contains("error"));
}

#[test]
fn e2e_unknown_overlay_exits_with_error() {
    let output = agenticenti()
        .args(["compose", "coder", "nonexistent-overlay"])
        .output()
        .unwrap();

    assert!(!output.status.success());
}

#[test]
fn e2e_exit_code_zero_on_success() {
    let output = agenticenti().args(["compose", "coder"]).output().unwrap();
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn e2e_exit_code_nonzero_on_failure() {
    let output = agenticenti().args(["compose", "fake"]).output().unwrap();
    assert_ne!(output.status.code(), Some(0));
}
