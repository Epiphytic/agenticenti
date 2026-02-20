use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::generated::prompts;

/// Categorises the three kinds of prompt files.
#[derive(Debug, Clone, Copy)]
pub enum PromptCategory {
    Role,
    Overlay,
    TestingMode,
    Appendix,
}

impl PromptCategory {
    fn dir_name(&self) -> &'static str {
        match self {
            PromptCategory::Role => "roles",
            PromptCategory::Overlay => "overlays",
            PromptCategory::TestingMode => "testing-modes",
            PromptCategory::Appendix => "appendices",
        }
    }

    fn embedded_lookup(&self, name: &str) -> Option<&'static str> {
        match self {
            PromptCategory::Role => prompts::embedded_role(name),
            PromptCategory::Overlay => prompts::embedded_overlay(name),
            PromptCategory::TestingMode => prompts::embedded_testing_mode(name),
            PromptCategory::Appendix => prompts::embedded_appendix(name),
        }
    }

    fn embedded_names(&self) -> &'static [&'static str] {
        match self {
            PromptCategory::Role => prompts::all_role_names(),
            PromptCategory::Overlay => prompts::all_overlay_names(),
            PromptCategory::TestingMode => prompts::all_testing_mode_names(),
            PromptCategory::Appendix => prompts::all_appendix_names(),
        }
    }
}

/// Metadata about a prompt that is available for composition.
#[derive(Debug)]
pub struct AvailablePrompt {
    pub name: String,
    pub source: PromptSource,
}

/// Where a prompt originates from.
#[derive(Debug, PartialEq)]
pub enum PromptSource {
    /// Shipped with the binary.
    Embedded,
    /// User file that shadows an embedded prompt.
    Override,
    /// User file with no embedded equivalent.
    UserOnly,
}

/// Error returned when a prompt cannot be resolved.
#[derive(Debug)]
pub struct ComposeError {
    pub category: String,
    pub name: String,
}

impl std::fmt::Display for ComposeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown {} '{}'", self.category, self.name)
    }
}

impl std::error::Error for ComposeError {}

/// Resolve a single prompt by name.
///
/// Resolution order:
/// 1. Check `config_dir/<category>/<name>.md` for a user override.
/// 2. Fall back to the compile-time embedded prompt.
/// 3. Return an error if neither exists.
pub fn resolve_prompt(
    config_dir: &Path,
    category: PromptCategory,
    name: &str,
) -> Result<String, ComposeError> {
    // Check override directory first
    let override_path = config_dir
        .join(category.dir_name())
        .join(format!("{}.md", name));

    if override_path.exists() {
        return fs::read_to_string(&override_path).map_err(|_| ComposeError {
            category: category.dir_name().to_string(),
            name: name.to_string(),
        });
    }

    // Fall back to embedded
    category
        .embedded_lookup(name)
        .map(|s| s.to_string())
        .ok_or_else(|| ComposeError {
            category: category.dir_name().to_string(),
            name: name.to_string(),
        })
}

/// Compose a full system prompt from a role, zero or more overlays, an
/// optional testing mode, and automatic appendices. Each resolved section
/// is joined with `\n\n---\n\n`.
///
/// The artifacts appendix is always appended. The beads appendix is appended
/// only when `include_beads` is true (auto-detected from `.beads/` directory
/// or forced via CLI flags).
pub fn compose(
    role: &str,
    overlays: &[String],
    testing_mode: Option<&str>,
    config_dir: &Path,
    include_beads: bool,
) -> Result<String, ComposeError> {
    let mut parts = vec![resolve_prompt(config_dir, PromptCategory::Role, role)?];

    for overlay in overlays {
        parts.push(resolve_prompt(
            config_dir,
            PromptCategory::Overlay,
            overlay,
        )?);
    }

    if let Some(mode) = testing_mode {
        parts.push(resolve_prompt(
            config_dir,
            PromptCategory::TestingMode,
            mode,
        )?);
    }

    // Appendices: always include artifacts, conditionally include beads
    parts.push(resolve_prompt(
        config_dir,
        PromptCategory::Appendix,
        "artifacts",
    )?);

    if include_beads {
        parts.push(resolve_prompt(
            config_dir,
            PromptCategory::Appendix,
            "beads",
        )?);
    }

    Ok(parts.join("\n\n---\n\n"))
}

/// List all available prompts for a category, merging embedded and user-provided
/// files. The results are sorted alphabetically by name.
pub fn list_available(category: PromptCategory, config_dir: &Path) -> Vec<AvailablePrompt> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    // Add embedded prompts
    for name in category.embedded_names() {
        let override_path = config_dir
            .join(category.dir_name())
            .join(format!("{}.md", name));

        let source = if override_path.exists() {
            PromptSource::Override
        } else {
            PromptSource::Embedded
        };

        results.push(AvailablePrompt {
            name: name.to_string(),
            source,
        });
        seen.insert(name.to_string());
    }

    // Add user-only prompts from config dir
    let user_dir = config_dir.join(category.dir_name());
    if let Ok(read_dir) = fs::read_dir(&user_dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                if !seen.contains(&name) {
                    results.push(AvailablePrompt {
                        name,
                        source: PromptSource::UserOnly,
                    });
                }
            }
        }
    }

    results.sort_by(|a, b| a.name.cmp(&b.name));
    results
}
