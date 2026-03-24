# agenticenti

Composable agent prompt CLI. Assemble system prompts for AI coding agents from modular roles, language overlays, and testing modes.

**Problem:** Agent prompts are long, repetitive, and hard to maintain. Teams copy-paste thousands of lines between configs, and prompts drift out of sync.

**Solution:** `agenticenti` composes prompts from small, reusable markdown files. Pick a role, add language overlays, and get a single prompt on stdout - ready to pipe into any agent framework.

## Quick Start

### Install from source (Rust)

```bash
cargo install --git https://github.com/Epiphytic/agenticenti.git
```

### Install via npm (no Rust toolchain needed)

```bash
npx @epiphytic/agenticenti compose coder rust
```

### Build from source

```bash
git clone https://github.com/Epiphytic/agenticenti.git
cd agenticenti
cargo build --release
```

## Usage

### Compose a prompt

Combine a base role with one or more language/stack overlays:

```bash
# Coder role with Rust overlay
agenticenti compose coder rust

# Coder role with multiple overlays
agenticenti compose coder rust docker

# Tester role with testing mode
agenticenti compose tester rust --testing-mode unit

# Tester with E2E testing mode and multiple stacks
agenticenti compose tester python terraform --testing-mode e2e

# Role with no overlays
agenticenti compose team-leader
```

Output goes to stdout as composed markdown, separated by `---` between sections. Pipe it wherever you need:

```bash
# Write to a file
agenticenti compose coder rust > system-prompt.md

# Use with Claude Code
agenticenti compose reviewer rust > .claude/reviewer-prompt.md
```

### List available prompts

```bash
# List everything
agenticenti list

# List only roles
agenticenti list --roles

# List only language/stack overlays
agenticenti list --overlays

# List only testing modes
agenticenti list --testing-modes
```

### Beads integration

`agenticenti` auto-detects the [beads](https://github.com/steveyegge/beads) issue tracker. If a `.beads/` directory exists in the current directory, the beads appendix is included automatically. Override with flags:

```bash
# Force include beads appendix
agenticenti compose coder rust --beads

# Force exclude beads appendix
agenticenti compose coder rust --no-beads
```

### Custom config directory

Override the default config directory (`$HOME/.agenticenti`):

```bash
agenticenti --config-dir /path/to/prompts compose coder rust
```

## Available Prompts

### Roles (15)

| Role | Description |
|------|-------------|
| `architect` | System design and architecture decisions |
| `coder` | Autonomous code implementation |
| `devops` | Infrastructure and deployment |
| `docs` | Technical documentation |
| `evangelist` | Developer advocacy and technical communication |
| `github-actions` | CI/CD workflow authoring |
| `integrator` | Cross-system integration |
| `maintainer` | Codebase health and dependency management |
| `planner` | Implementation planning and task breakdown |
| `researcher` | Technical research and analysis |
| `reviewer` | Code review |
| `security-reviewer` | Security-focused code review |
| `team-leader` | Agent team coordination (orchestration layer) |
| `tester` | Test authoring and quality assurance |
| `troubleshooter` | Debugging and root cause analysis |

### Language/Stack Overlays (7)

`bash`, `docker`, `go`, `python`, `rust`, `terraform`, `typescript-node`

### Testing Modes (2)

`unit`, `e2e`

### Appendices (auto-included)

- **artifacts** - always included; defines the `docs/` artifact directory structure
- **beads** - included when `.beads/` directory is detected or `--beads` flag is set

## Customization

### Override embedded prompts

Create files in `$HOME/.agenticenti/` (or your `--config-dir`) that mirror the embedded structure:

```
$HOME/.agenticenti/
  roles/
    coder.md          # Replaces the built-in coder role
  overlays/
    rust.md           # Replaces the built-in Rust overlay
  testing-modes/
    unit.md           # Replaces the built-in unit testing mode
```

Override files fully replace the embedded prompt (no merging).

### Add custom prompts

Drop new `.md` files into the config directory to create prompts that don't exist in the embedded set:

```bash
mkdir -p ~/.agenticenti/roles
echo "# Role: My Custom Role\n\nCustom instructions here." > ~/.agenticenti/roles/my-custom-role.md

agenticenti compose my-custom-role rust
agenticenti list --roles  # Shows "my-custom-role (user)"
```

Custom prompts appear with a `(user)` tag in `list` output. Overridden prompts appear with `(override)`.

## Architecture

```
                    +-------------------------------+
                    |     agenticenti CLI (clap)     |
                    |                               |
 User runs:        |  1. Parse args                 |
 agenticenti       |  2. For each piece:            |
   compose         |     a. Check config dir        |
   coder rust      |     b. Override exists? Use it |
                    |     c. Else use embedded       |
                    |  3. Join with --- separators   |
                    |  4. Print to stdout            |
                    +-------------------------------+
                              |
              +---------------+---------------+
              v               v               v
     +--------------+ +--------------+ +--------------+
     | Embedded     | | Embedded     | | Embedded     |
     | Roles (15)   | | Overlays (7) | | Test Modes(2)|
     | (build.rs)   | | (build.rs)   | | (build.rs)   |
     +--------------+ +--------------+ +--------------+
```

### Prompt composition order

```
[Base Role]  ---  [Overlay 1]  ---  [Overlay 2]  ---  [Testing Mode]  ---  [Artifacts Appendix]  ---  [Beads Appendix]
```

### How embedding works

`build.rs` runs at compile time:

1. Walks `prompts/roles/`, `prompts/overlays/`, `prompts/testing-modes/`, `prompts/appendices/`
2. Generates `include_str!()` constants for each `.md` file
3. Generates lookup functions (`embedded_role(name)`, `embedded_overlay(name)`, etc.)
4. Generates name-listing functions (`all_role_names()`, `all_overlay_names()`, etc.)

The generated module is included at `src/lib.rs` via `include!(concat!(env!("OUT_DIR"), "/prompts.rs"))`.

### Resolution logic

For each prompt piece requested:

1. Check `<config_dir>/<category>/<name>.md` for a user file
2. If found, use it (full replacement)
3. Otherwise, use the compile-time embedded prompt
4. If neither exists, return an error

### Key source files

| File | Purpose |
|------|---------|
| `src/main.rs` | CLI entry point, argument parsing with clap |
| `src/lib.rs` | Library root, re-exports `composer` and `generated::prompts` |
| `src/composer.rs` | Prompt resolution, composition, and listing |
| `build.rs` | Compile-time prompt discovery and code generation |
| `prompts/` | Embedded prompt markdown files |

## WASM/npm Distribution

`agenticenti` compiles to `wasm32-wasip1` for distribution via npm without requiring a Rust toolchain on the consumer side. See [ADR-001](docs/adrs/001-wasm-npm-distribution.md) for the full decision record.

```bash
# Build WASM
make build-wasm

# Publish to npm
make npm-publish
```

The npm package runs the WASM binary through Node.js's built-in `node:wasi` module (Node 18+).

## Development

```bash
# Run tests
cargo test

# Build native binary
cargo build --release

# Build WASM target
make build-wasm

# Generate and view API docs
cargo doc --no-deps --open
```

### Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Unknown role, overlay, or testing mode |
| 2 | Invalid arguments (from clap) |

## License

MIT
