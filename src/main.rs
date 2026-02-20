use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

use agenticenti::composer::{self, PromptCategory, PromptSource};

#[derive(Parser)]
#[command(name = "agenticenti", about = "Composable agent prompt CLI")]
struct Cli {
    /// Override config directory (default: $HOME/.agenticenti)
    #[arg(long, global = true)]
    config_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compose a prompt from role + language overlays
    Compose {
        /// The base role (e.g., coder, tester, reviewer)
        role: String,

        /// Language/stack overlays to append (e.g., rust, python, docker)
        languages: Vec<String>,

        /// Testing mode overlay (unit or e2e) — typically used with the tester role
        #[arg(long)]
        testing_mode: Option<String>,
    },

    /// List available roles, overlays, or testing modes
    List {
        /// Show available roles
        #[arg(long)]
        roles: bool,

        /// Show available language/stack overlays
        #[arg(long)]
        overlays: bool,

        /// Show available testing mode overlays
        #[arg(long)]
        testing_modes: bool,
    },
}

fn default_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".agenticenti")
}

fn main() {
    let cli = Cli::parse();
    let config_dir = cli.config_dir.unwrap_or_else(default_config_dir);

    match cli.command {
        Commands::Compose {
            role,
            languages,
            testing_mode,
        } => match composer::compose(&role, &languages, testing_mode.as_deref(), &config_dir) {
            Ok(prompt) => print!("{}", prompt),
            Err(e) => {
                eprintln!("error: {}", e);
                // Exit code 1 = unknown role/overlay. Clap uses exit code 2 for invalid arguments.
                process::exit(1);
            }
        },
        Commands::List {
            roles,
            overlays,
            testing_modes,
        } => {
            // Default: show all if none specified
            let show_all = !roles && !overlays && !testing_modes;

            if roles || show_all {
                println!("Roles:");
                for p in composer::list_available(PromptCategory::Role, &config_dir) {
                    let tag = match p.source {
                        PromptSource::Embedded => "",
                        PromptSource::Override => " (override)",
                        PromptSource::UserOnly => " (user)",
                    };
                    println!("  {}{}", p.name, tag);
                }
            }

            if overlays || show_all {
                if roles || show_all {
                    println!();
                }
                println!("Overlays:");
                for p in composer::list_available(PromptCategory::Overlay, &config_dir) {
                    let tag = match p.source {
                        PromptSource::Embedded => "",
                        PromptSource::Override => " (override)",
                        PromptSource::UserOnly => " (user)",
                    };
                    println!("  {}{}", p.name, tag);
                }
            }

            if testing_modes || show_all {
                if roles || overlays || show_all {
                    println!();
                }
                println!("Testing Modes:");
                for p in composer::list_available(PromptCategory::TestingMode, &config_dir) {
                    let tag = match p.source {
                        PromptSource::Embedded => "",
                        PromptSource::Override => " (override)",
                        PromptSource::UserOnly => " (user)",
                    };
                    println!("  {}{}", p.name, tag);
                }
            }
        }
    }
}
