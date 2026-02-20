use std::path::PathBuf;

use agenticenti::composer::{
    compose, list_available, resolve_prompt, PromptCategory, PromptSource,
};

#[test]
fn test_resolve_embedded_role() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = resolve_prompt(&config_dir, PromptCategory::Role, "coder");
    assert!(result.is_ok());
    let content = result.unwrap();
    assert!(!content.is_empty());
}

#[test]
fn test_resolve_unknown_role_fails() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = resolve_prompt(&config_dir, PromptCategory::Role, "nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_resolve_override_takes_precedence() {
    let tmp = tempfile::tempdir().unwrap();
    let roles_dir = tmp.path().join("roles");
    std::fs::create_dir_all(&roles_dir).unwrap();
    std::fs::write(roles_dir.join("coder.md"), "OVERRIDE CONTENT").unwrap();

    let result = resolve_prompt(tmp.path(), PromptCategory::Role, "coder");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "OVERRIDE CONTENT");
}

#[test]
fn test_compose_role_only() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &[], None, &config_dir, false);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.is_empty());
    // Has separator between role and artifacts appendix
    let separator_count = output.matches("\n\n---\n\n").count();
    assert_eq!(separator_count, 1);
}

#[test]
fn test_compose_role_plus_overlay() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &["rust".to_string()], None, &config_dir, false);
    assert!(result.is_ok());
    let output = result.unwrap();
    // role + overlay + artifacts = 2 separators
    let separator_count = output.matches("\n\n---\n\n").count();
    assert_eq!(separator_count, 2);
}

#[test]
fn test_compose_role_plus_multiple_overlays() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose(
        "coder",
        &["rust".to_string(), "docker".to_string()],
        None,
        &config_dir,
        false,
    );
    assert!(result.is_ok());
    let output = result.unwrap();
    // role + overlay1 + overlay2 + artifacts = 3 separators
    let separator_count = output.matches("\n\n---\n\n").count();
    assert_eq!(separator_count, 3);
}

#[test]
fn test_compose_with_testing_mode() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose(
        "tester",
        &["rust".to_string()],
        Some("unit"),
        &config_dir,
        false,
    );
    assert!(result.is_ok());
    let output = result.unwrap();
    // role + overlay + testing mode + artifacts = 3 separators
    let separator_count = output.matches("\n\n---\n\n").count();
    assert_eq!(separator_count, 3);
}

#[test]
fn test_compose_unknown_role_fails() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("nonexistent", &[], None, &config_dir, false);
    assert!(result.is_err());
}

#[test]
fn test_compose_unknown_overlay_fails() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose(
        "coder",
        &["nonexistent".to_string()],
        None,
        &config_dir,
        false,
    );
    assert!(result.is_err());
}

#[test]
fn test_compose_includes_artifacts_appendix() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &[], None, &config_dir, false);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Artifact Directory Structure"));
}

#[test]
fn test_compose_includes_beads_when_flagged() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &[], None, &config_dir, true);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Beads Issue Tracking"));
    // role + artifacts + beads = 2 separators
    let separator_count = output.matches("\n\n---\n\n").count();
    assert_eq!(separator_count, 2);
}

#[test]
fn test_compose_excludes_beads_when_not_flagged() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &[], None, &config_dir, false);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(!output.contains("Beads Issue Tracking"));
}

#[test]
fn test_list_available_includes_all_roles() {
    let config_dir = PathBuf::from("/nonexistent");
    let roles = list_available(PromptCategory::Role, &config_dir);
    assert!(roles.len() >= 15);
    assert!(roles.iter().any(|r| r.name == "coder"));
    assert!(roles.iter().any(|r| r.name == "evangelist"));
    assert!(roles.iter().any(|r| r.name == "github-actions"));
}

#[test]
fn test_list_available_includes_all_overlays() {
    let config_dir = PathBuf::from("/nonexistent");
    let overlays = list_available(PromptCategory::Overlay, &config_dir);
    assert!(overlays.len() >= 7);
    assert!(overlays.iter().any(|o| o.name == "rust"));
    assert!(overlays.iter().any(|o| o.name == "bash"));
    assert!(overlays.iter().any(|o| o.name == "docker"));
}

#[test]
fn test_list_shows_user_only_prompts() {
    let tmp = tempfile::tempdir().unwrap();
    let roles_dir = tmp.path().join("roles");
    std::fs::create_dir_all(&roles_dir).unwrap();
    std::fs::write(roles_dir.join("custom-role.md"), "Custom content").unwrap();

    let roles = list_available(PromptCategory::Role, tmp.path());
    let custom = roles.iter().find(|r| r.name == "custom-role");
    assert!(custom.is_some());
    assert_eq!(custom.unwrap().source, PromptSource::UserOnly);
}
