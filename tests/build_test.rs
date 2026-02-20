#[test]
fn test_embedded_roles_exist() {
    let roles = agenticenti::generated::prompts::all_role_names();
    assert!(roles.contains(&"coder"));
    assert!(roles.contains(&"tester"));
    assert!(roles.contains(&"evangelist"));
    assert!(roles.contains(&"github-actions"));
    assert!(roles.len() >= 15);
}

#[test]
fn test_embedded_overlays_exist() {
    let overlays = agenticenti::generated::prompts::all_overlay_names();
    assert!(overlays.contains(&"rust"));
    assert!(overlays.contains(&"bash"));
    assert!(overlays.contains(&"docker"));
    assert!(overlays.len() >= 7);
}

#[test]
fn test_embedded_testing_modes_exist() {
    let modes = agenticenti::generated::prompts::all_testing_mode_names();
    assert!(modes.contains(&"unit"));
    assert!(modes.contains(&"e2e"));
    assert_eq!(modes.len(), 2);
}

#[test]
fn test_role_content_is_not_empty() {
    let content = agenticenti::generated::prompts::embedded_role("coder");
    assert!(content.is_some());
    assert!(!content.unwrap().is_empty());
}

#[test]
fn test_unknown_role_returns_none() {
    let content = agenticenti::generated::prompts::embedded_role("nonexistent");
    assert!(content.is_none());
}

#[test]
fn test_embedded_appendices_exist() {
    let appendices = agenticenti::generated::prompts::all_appendix_names();
    assert!(appendices.contains(&"artifacts"));
    assert!(appendices.contains(&"beads"));
    assert_eq!(appendices.len(), 2);
}

#[test]
fn test_appendix_content_is_not_empty() {
    let content = agenticenti::generated::prompts::embedded_appendix("artifacts");
    assert!(content.is_some());
    assert!(!content.unwrap().is_empty());

    let content = agenticenti::generated::prompts::embedded_appendix("beads");
    assert!(content.is_some());
    assert!(!content.unwrap().is_empty());
}
