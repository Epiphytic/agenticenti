//! Composable agent prompt library.
//!
//! `agenticenti` composes system prompts for AI coding agents from modular
//! markdown files: a base role, zero or more language/stack overlays, an
//! optional testing mode, and automatic appendices.
//!
//! # Prompt resolution
//!
//! Each prompt piece resolves through a two-tier lookup:
//!
//! 1. Check the user's config directory (`$HOME/.agenticenti/` by default) for
//!    an override file.
//! 2. Fall back to the compile-time embedded prompt (baked in via `build.rs`).
//!
//! # Example
//!
//! ```rust
//! use std::path::PathBuf;
//! use agenticenti::composer::{compose, PromptCategory, list_available};
//!
//! let config = PathBuf::from("/nonexistent");
//!
//! // Compose a coder prompt with a Rust overlay
//! let prompt = compose("coder", &["rust".into()], None, &config, false).unwrap();
//! assert!(prompt.contains("---"));
//!
//! // List available roles
//! let roles = list_available(PromptCategory::Role, &config);
//! assert!(roles.iter().any(|r| r.name == "coder"));
//! ```

/// Prompt resolution, composition, and listing.
pub mod composer;

/// Auto-generated prompt constants and lookup functions.
///
/// Built by `build.rs` at compile time from the `prompts/` directory.
/// Contains one sub-module per category (`roles`, `overlays`,
/// `testing_modes`, `appendices`) with `&'static str` constants, plus
/// lookup functions like `embedded_role()` and `all_role_names()`.
pub mod generated {
    /// Embedded prompt data and lookup functions.
    pub mod prompts {
        include!(concat!(env!("OUT_DIR"), "/prompts.rs"));
    }
}
