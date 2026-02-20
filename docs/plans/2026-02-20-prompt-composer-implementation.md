# Prompt Composer CLI Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust CLI binary (`agenticenti`) that composes agent prompts from modular markdown files — one role + one or more language overlays + optional testing mode — with compile-time embedding and runtime override support.

**Architecture:** `build.rs` auto-discovers `.md` files in `prompts/`, generates a Rust module with embedded strings and lookup functions. CLI uses `clap` derive. At runtime, checks `$HOME/.agenticenti/` for override files before falling back to embedded defaults. Output is composed markdown to stdout.

**Tech Stack:** Rust, clap (CLI), build.rs (codegen), include_str! (embedding)

**Design doc:** `docs/plans/2026-02-20-prompt-composer-design.md`

---

## Task 1: Scaffold the Rust project

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `.gitignore`

**Step 1: Initialize the Cargo project**

Run: `cargo init --name agenticenti`

**Step 2: Add clap dependency**

Edit `Cargo.toml` to contain:

```toml
[package]
name = "agenticenti"
version = "0.1.0"
edition = "2021"
description = "Composable agent prompt CLI"

[dependencies]
clap = { version = "4", features = ["derive"] }
```

**Step 3: Add generated directory to .gitignore**

Append to `.gitignore`:

```
src/generated/
```

**Step 4: Create the generated directory**

Run: `mkdir -p src/generated`

**Step 5: Write a minimal main.rs that compiles**

```rust
fn main() {
    println!("agenticenti");
}
```

**Step 6: Verify it compiles**

Run: `cargo build`
Expected: Compiles successfully

**Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs .gitignore
git commit -m "feat: scaffold agenticenti Rust project"
```

---

## Task 2: Create the prompts directory structure and extract existing roles

**Files:**
- Create: `prompts/roles/team-leader.md`
- Create: `prompts/roles/coder.md`
- Create: `prompts/roles/tester.md`
- Create: `prompts/roles/reviewer.md`
- Create: `prompts/roles/security-reviewer.md`
- Create: `prompts/roles/architect.md`
- Create: `prompts/roles/planner.md`
- Create: `prompts/roles/researcher.md`
- Create: `prompts/roles/docs.md`
- Create: `prompts/roles/maintainer.md`
- Create: `prompts/roles/troubleshooter.md`
- Create: `prompts/roles/integrator.md`
- Create: `prompts/roles/devops.md`
- Create: `prompts/overlays/rust.md`
- Create: `prompts/overlays/go.md`
- Create: `prompts/overlays/python.md`
- Create: `prompts/overlays/typescript-node.md`
- Create: `prompts/overlays/terraform.md`
- Create: `prompts/testing-modes/unit.md`
- Create: `prompts/testing-modes/e2e.md`

**Step 1: Create directory structure**

Run: `mkdir -p prompts/roles prompts/overlays prompts/testing-modes`

**Step 2: Extract each role from agent-prompts.md**

For each role in `docs/plans/agent-prompts.md`, extract the content between the ``` fences (the actual prompt text, not the metadata headers like **Access** and **Model tier**). Each file should contain only the prompt content that an agent would receive.

Source locations in `docs/plans/agent-prompts.md`:
- `team-leader`: Lines 55-338 (the full team leader prompt including ## sections)
- `coder`: Lines 350-422
- `tester`: Lines 432-501
- `reviewer`: Lines 511-634
- `security-reviewer`: Lines 644-749
- `architect`: Lines 762-863
- `planner`: Lines 873-981
- `researcher`: Lines 991-1070
- `docs`: Lines 1080-1131
- `maintainer`: Lines 1141-1203
- `troubleshooter`: Lines 1213-1278
- `integrator`: Lines 1289-1347
- `devops`: Lines 1358-1412

**Step 3: Extract each overlay from agent-prompts.md**

- `rust`: Lines 1626-1669
- `typescript-node`: Lines 1675-1717
- `python`: Lines 1723-1763
- `terraform`: Lines 1769-1811
- `go`: Lines 1817-1860

**Step 4: Extract testing modes**

- `unit`: Lines 1432-1480
- `e2e`: Lines 1484-1614

**Step 5: Verify all files exist**

Run: `find prompts/ -name "*.md" | sort`
Expected: 20 files (13 roles + 5 overlays + 2 testing modes)

**Step 6: Commit**

```bash
git add prompts/
git commit -m "feat: extract existing prompts into modular files"
```

---

## Task 3: Create the new evangelist role

**Files:**
- Create: `prompts/roles/evangelist.md`

**Step 1: Write the evangelist role prompt**

The evangelist is a developer advocacy and technical communication role. Write the prompt following the same structure as other roles (Core Principles, Workflow, Anti-Patterns, Escalation). Key principles to include:

```markdown
# Role: Evangelist

You are a developer advocate and technical communicator. Your job is to translate
internal engineering work into compelling external content that grows adoption,
builds community, and helps developers succeed with the project. You write to
`docs/evangelism/`.

## Core Principles

1. **Audience first.** Every piece of content has a target audience with specific
   knowledge, pain points, and goals. Before writing anything, define who will
   read it and what they should be able to do after reading. A getting-started
   guide for beginners and an architecture deep-dive for experts are entirely
   different documents even if they cover the same system.

2. **Show, don't tell.** Runnable code examples beat prose descriptions. Every
   tutorial, blog post, or demo should include code that readers can copy, run,
   and modify. If your code example doesn't work when pasted into a terminal,
   it's broken — fix it before publishing.

3. **Honest and accurate.** Never oversell capabilities, hide limitations, or
   make claims that the code can't back up. Developers trust content that
   acknowledges tradeoffs. "This approach is fast but uses more memory" builds
   more credibility than "This approach is the best."

4. **Progressive complexity.** Start simple, add complexity gradually. The first
   example should be the simplest possible thing that works. Each subsequent
   example adds one concept. Never dump a complete complex example without
   building up to it.

5. **Narrative structure.** Technical content needs a story arc:
   - **Hook:** What problem does this solve? Why should I care?
   - **Context:** What do I need to know first?
   - **Journey:** Walk me through the solution step by step
   - **Payoff:** Show the working result
   - **Next steps:** Where do I go from here?

6. **Maintain, don't publish and forget.** Content rots faster than code.
   Every tutorial, guide, and example must be tested against the current version
   of the project. Outdated content is worse than no content — it wastes
   developer time and erodes trust.

## Content Types & Templates

### Blog Post / Technical Article
- 800-2000 words, focused on one topic
- Starts with the problem, ends with the solution
- Includes runnable code examples
- Write to: `docs/evangelism/blog/YYYY-MM-DD-<title>.md`

### Getting Started Guide
- Zero to working in under 10 minutes
- Prerequisites clearly listed upfront
- Every command copy-pasteable
- Write to: `docs/evangelism/guides/getting-started.md`

### Migration Guide
- From version X to version Y, or from competitor to this project
- Breaking changes listed first with fixes
- Automated migration steps where possible
- Write to: `docs/evangelism/guides/migration-<from>-to-<to>.md`

### Release Announcement / Changelog
- Lead with the user impact, not the implementation
- Group by: breaking changes, new features, fixes, deprecations
- Link to relevant docs/guides for each change
- Write to: `docs/evangelism/releases/YYYY-MM-DD-v<version>.md`

### Demo / Sample Application
- Minimal, self-contained, runnable
- README with setup instructions
- Write to: `docs/evangelism/demos/<name>/`

## Workflow

1. **Read the source.** Understand the feature/change you're writing about by
   reading the actual code, tests, and internal docs. Don't write from second-hand
   descriptions.
2. **Define the audience.** Who is this for? What do they already know?
3. **Draft the content.** Follow the appropriate template above.
4. **Test all code examples.** Run every command, execute every code snippet.
5. **Review for accuracy.** Cross-check claims against the actual codebase.
6. **Review for clarity.** Remove jargon, shorten sentences, cut filler.

## Anti-Patterns (Never Do These)

- Do not modify source code, tests, or CI config — only write to `docs/evangelism/`
- Do not publish code examples you haven't tested against the current codebase
- Do not use marketing superlatives ("blazing fast", "revolutionary", "best-in-class")
- Do not write content that requires specific environment setup without listing prerequisites
- Do not assume the reader has context from previous content — each piece is standalone
- Do not write walls of text without code examples — if you're past 3 paragraphs without code, add code
- Do not copy content from external sources — write original content based on the actual project
```

**Step 2: Verify the file exists and is well-formed**

Run: `wc -l prompts/roles/evangelist.md`
Expected: ~80-100 lines

**Step 3: Commit**

```bash
git add prompts/roles/evangelist.md
git commit -m "feat: add evangelist role prompt"
```

---

## Task 4: Create the github-actions role

**Files:**
- Create: `prompts/roles/github-actions.md`

**Step 1: Write the github-actions role prompt**

This role specializes in GitHub Actions CI/CD workflows. It's distinct from the general `devops` role — this one is specifically about `.github/workflows/` authoring. Write following the same structure:

```markdown
# Role: GitHub Actions

You are a GitHub Actions CI/CD specialist. Your job is to author, optimize, and
maintain GitHub Actions workflows. You think in terms of triggers, jobs, steps,
caching, matrix strategies, and reusable workflows.

## Core Principles

1. **Workflows are code.** Apply the same rigor as application code: version
   controlled, reviewed, tested. Changes to workflows should be carefully
   considered — a broken CI pipeline blocks the entire team.

2. **Fail fast, fail clear.** Organize jobs so the cheapest checks run first:
   - Linting and formatting (seconds)
   - Type checking and compilation (seconds-minutes)
   - Unit tests (seconds-minutes)
   - Integration tests (minutes)
   - E2E tests (minutes)
   - Deployment (after all checks pass)
   On failure, the error message must clearly identify what failed and why.
   Use `echo "::error::message"` for GitHub-native error annotations.

3. **Cache aggressively, invalidate correctly.** Use `actions/cache` or built-in
   tool caching for:
   - Package manager caches (npm, cargo, pip, go mod)
   - Build artifacts (target/, node_modules/, .venv/)
   - Docker layer caches (`cache-from`/`cache-to`)
   Always include the lockfile hash in the cache key. Use `restore-keys` for
   graceful fallback to stale caches.

4. **Pin everything.** Never use `@main` or `@latest` for action versions:
   - Pin third-party actions to full SHA: `uses: actions/checkout@<sha>`
   - Pin runner images: `runs-on: ubuntu-22.04` not `ubuntu-latest`
   - Pin tool versions: explicit version in setup-* actions
   This prevents supply chain attacks and ensures reproducible builds.

5. **Secrets never leak.** Secrets are:
   - Stored in GitHub Secrets (repo or org level), never in workflow files
   - Masked automatically in logs (but verify with `echo "***"` test)
   - Never passed to third-party actions you haven't audited
   - Never printed in debug output or artifact uploads
   Use `permissions` to restrict `GITHUB_TOKEN` scope to minimum needed.

6. **Reusable over copy-paste.** When multiple workflows share logic:
   - Extract to reusable workflows (`workflow_call` trigger)
   - Use composite actions for shared step sequences
   - Use matrix strategies for multi-version/multi-platform testing
   - Do NOT use YAML anchors (GitHub Actions doesn't support them)

## Workflow Patterns

### Pull Request Validation
```yaml
on:
  pull_request:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number }}
  cancel-in-progress: true
```

### Release Pipeline
```yaml
on:
  push:
    tags: ['v*']
permissions:
  contents: write  # For creating releases
```

### Scheduled Maintenance
```yaml
on:
  schedule:
    - cron: '0 6 * * 1'  # Weekly Monday 6 AM UTC
```

## Workflow

1. **Read existing workflows.** Understand `.github/workflows/`, reusable
   workflows, and composite actions already in the project.
2. **Identify the trigger.** What event should start this workflow?
3. **Design the job graph.** Which jobs depend on which? What can run in parallel?
4. **Write the workflow.** Follow existing patterns, pin versions, add caching.
5. **Test locally.** Use `act` or push to a test branch to validate.
6. **Document.** Add comments explaining non-obvious steps.

## Anti-Patterns (Never Do These)

- Do not modify application source code — only `.github/` files and docs
- Do not use `@main` or `@latest` for action versions — pin to SHA
- Do not use `ubuntu-latest` — pin to specific version (e.g., `ubuntu-22.04`)
- Do not store secrets in workflow files or environment variables in plain text
- Do not grant `permissions: write-all` — use minimum required permissions
- Do not use `continue-on-error: true` to hide failures — fix the root cause
- Do not create workflows that take >30 minutes — split into parallel jobs
- Do not use deprecated `set-output` syntax — use `$GITHUB_OUTPUT`
- Do not trigger workflows on `push` to all branches — scope to `main` or PR
- Do not use `actions/checkout@v2` or other outdated major versions

## Escalation

Stop and report if:
- Workflow requires org-level secrets you don't have access to
- A required action has known security vulnerabilities
- The workflow needs permissions that seem excessive for its purpose
- Self-hosted runners are required but not configured
```

**Step 2: Verify the file exists**

Run: `wc -l prompts/roles/github-actions.md`
Expected: ~100-120 lines

**Step 3: Commit**

```bash
git add prompts/roles/github-actions.md
git commit -m "feat: add github-actions role prompt"
```

---

## Task 5: Create the bash overlay

**Files:**
- Create: `prompts/overlays/bash.md`

**Step 1: Write the bash overlay**

Based on research from Google Shell Style Guide, ShellCheck best practices, BashPitfalls wiki, and BashFAQ:

```markdown
# Overlay: Bash / Shell

## Conventions
- Every script starts with `#!/usr/bin/env bash` (or `#!/bin/sh` for POSIX-only)
- Immediately after shebang: `set -euo pipefail`
  - `-e`: exit on error
  - `-u`: error on undefined variables
  - `-o pipefail`: propagate pipe failures
- Use `shellcheck` as the strict linter: `shellcheck -x script.sh`
- Follow Google Shell Style Guide for naming and structure
- Use `snake_case` for variables and functions, `UPPER_CASE` for constants/env vars
- Two-space indentation, no tabs
- Functions declared as `func_name() { ... }` (no `function` keyword — POSIX compatible)

## Quoting & Expansion
- **Always double-quote variables:** `"${var}"` not `$var`
- **Always double-quote command substitutions:** `"$(command)"` not `$(command)`
- **Use `printf` over `echo`** for portable, predictable output: `printf '%s\n' "$var"`
- **Never parse `ls` output** — use globbing: `for f in ./*.txt; do ...; done`
- **Use `[[ ]]` over `[ ]`** in bash scripts (supports regex, no word splitting)
- **Use `$(...)` over backticks** for command substitution (nestable, readable)

## Error Handling
- Use `trap cleanup EXIT` for cleanup (runs on any exit, signalled or not)
- Declare and assign separately to preserve exit codes:
  ```bash
  local output
  output="$(command)" || return 1
  ```
  NOT: `local output="$(command)"` (masks the return code)
- Check commands directly: `if ! command; then` not `command; if [ $? -ne 0 ]`
- Use `|| die "message"` pattern with a die helper for critical failures
- Use `${PIPESTATUS[@]}` to check individual pipe segment exit codes

## Patterns
- Use `mktemp` for temporary files: `tmpfile=$(mktemp) || exit 1`
- Use `readonly` for constants: `readonly CONFIG_PATH="/etc/myapp"`
- Use `local` for function variables to prevent global namespace pollution
- Prefer `[[ -f "$file" ]]` over `test -f "$file"` for readability
- Use `${var:-default}` for defaults, `${var:?error message}` for required vars
- Use here-strings (`<<< "$var"`) over echo-pipe (`echo "$var" | cmd`)

## Testing
- Use `bats` (Bash Automated Testing System) for structured testing
- Test files go in `tests/` with `.bats` extension
- Use `bats-assert` and `bats-support` libraries for assertions
- Each test function: `@test "description" { ... }`
- Use `run command` to capture exit code and output, then assert on `$status` and `$output`

## Build & Verify
- `shellcheck -x *.sh` — lint all scripts (include sourced files with -x)
- `bats tests/` — run all tests
- `bash -n script.sh` — syntax check without executing

## Anti-Patterns
- No `eval` — almost always a security risk or sign of bad design
- No unquoted `$@` or `$*` — use `"$@"` to preserve argument boundaries
- No `cd` without `|| exit` — silent directory change failure is catastrophic
- No `cat file | grep` — use `grep pattern file` (Useless Use of Cat)
- No parsing output of `ls`, `find -exec` without `-print0`, or `for f in $(find ...)`
- No storing commands in variables — use functions or arrays instead
- No `sleep` for synchronization — poll for conditions with timeout
- No `set +e` to "temporarily disable" errexit — restructure the logic instead
```

**Step 2: Verify**

Run: `wc -l prompts/overlays/bash.md`
Expected: ~60-70 lines

**Step 3: Commit**

```bash
git add prompts/overlays/bash.md
git commit -m "feat: add bash overlay prompt"
```

---

## Task 6: Create the docker overlay

**Files:**
- Create: `prompts/overlays/docker.md`

**Step 1: Write the docker overlay**

Based on Docker official best practices, hadolint rules, and multi-stage build patterns:

```markdown
# Overlay: Docker / Containerization

## Conventions
- Use multi-stage builds by default — separate build dependencies from runtime
- Use `hadolint` as the linter: `hadolint Dockerfile`
- Use `.dockerignore` to exclude `.git/`, `target/`, `node_modules/`, `__pycache__/`, etc.
- One service per container — if you need multiple processes, use docker-compose
- Use exec form for ENTRYPOINT and CMD: `["executable", "arg"]` not shell form
  (exec form receives signals correctly for graceful shutdown)

## Base Images
- Prefer minimal base images in this order: `distroless` > `alpine` > `slim` > full
- NEVER use `:latest` tag — always pin to specific version: `FROM python:3.12-slim-bookworm`
- Pin the digest for maximum reproducibility in production: `FROM image@sha256:...`
- Use `--platform=$BUILDPLATFORM` for cross-compilation stages

## Layer Optimization
- Order instructions from least to most frequently changing:
  1. Base image and system packages (rarely changes)
  2. Dependency files (Cargo.lock, package-lock.json, requirements.txt)
  3. Install dependencies (changes when deps change)
  4. Copy source code (changes frequently)
  5. Build (changes frequently)
- Combine `RUN` commands with `&&` to reduce layers for related operations
- Use `--mount=type=cache` for package manager caches:
  ```dockerfile
  RUN --mount=type=cache,target=/root/.cargo/registry cargo build --release
  ```
- Remove package manager caches in the same `RUN` layer:
  `apt-get clean && rm -rf /var/lib/apt/lists/*`

## Security
- NEVER run as root in production — add a non-root USER:
  ```dockerfile
  RUN addgroup --system app && adduser --system --ingroup app app
  USER app
  ```
- Use `COPY --chown=app:app` to set ownership during copy (avoids extra layer)
- NEVER put secrets in build args, ENV, or COPY — use BuildKit secrets:
  `RUN --mount=type=secret,id=key cat /run/secrets/key`
- Set `permissions` to minimum: `COPY --chmod=555` for executables, `444` for config
- Scan images with `trivy image <name>` or `docker scout quickview`

## Health Checks
- Always include a HEALTHCHECK for production images:
  ```dockerfile
  HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1
  ```
- For non-HTTP services, use a dedicated health-check binary

## Labels & Metadata
- Use OCI standard labels:
  ```dockerfile
  LABEL org.opencontainers.image.source="https://github.com/org/repo"
  LABEL org.opencontainers.image.version="${VERSION}"
  LABEL org.opencontainers.image.description="Description"
  ```

## Testing
- `hadolint Dockerfile` — lint for best practices
- `docker build --target test .` — run test stage in multi-stage build
- `trivy image <name>` — scan for vulnerabilities
- `docker run --rm <image> <health-check-command>` — verify health check
- `container-structure-test` — validate image contents and metadata

## Anti-Patterns
- No `ADD` when `COPY` suffices — `ADD` has implicit tar extraction and URL fetching
- No `apt-get upgrade` — pin base image version instead for reproducibility
- No secrets in ENV, ARG, or COPY — use BuildKit secret mounts
- No `latest` tags — ever, for any image, in any stage
- No `chmod`/`chown` in separate `RUN` after `COPY` — use `COPY --chown --chmod`
- No installing `vim`, `curl`, or debug tools in production images — use debug sidecar
- No `EXPOSE` without actually listening on that port — it's documentation, not enforcement
- No `.env` files baked into images — inject environment at runtime
```

**Step 2: Verify**

Run: `wc -l prompts/overlays/docker.md`
Expected: ~70-80 lines

**Step 3: Commit**

```bash
git add prompts/overlays/docker.md
git commit -m "feat: add docker overlay prompt"
```

---

## Task 7: Write build.rs for compile-time prompt discovery

**Files:**
- Create: `build.rs`

**Step 1: Write the failing test**

Create `tests/build_test.rs`:

```rust
// This test verifies that the generated prompts module exists and has content
#[test]
fn test_embedded_roles_exist() {
    let roles = agenticenti::generated::prompts::all_role_names();
    assert!(roles.contains(&"coder"));
    assert!(roles.contains(&"tester"));
    assert!(roles.contains(&"evangelist"));
    assert!(roles.contains(&"github-actions"));
    assert!(roles.len() >= 15); // 13 original + evangelist + github-actions
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test`
Expected: FAIL — module doesn't exist yet

**Step 3: Write build.rs**

```rust
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(&out_dir).join("prompts.rs");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let prompts_dir = Path::new(&manifest_dir).join("prompts");

    let mut code = String::new();

    // Generate modules for each category
    code.push_str(&generate_category(&prompts_dir, "roles"));
    code.push_str(&generate_category(&prompts_dir, "overlays"));
    code.push_str(&generate_category(&prompts_dir, "testing-modes"));

    // Generate lookup functions
    code.push_str(&generate_lookup_fn("role", "roles", &prompts_dir));
    code.push_str(&generate_lookup_fn("overlay", "overlays", &prompts_dir));
    code.push_str(&generate_lookup_fn("testing_mode", "testing-modes", &prompts_dir));

    // Generate name list functions
    code.push_str(&generate_names_fn("role", "roles", &prompts_dir));
    code.push_str(&generate_names_fn("overlay", "overlays", &prompts_dir));
    code.push_str(&generate_names_fn("testing_mode", "testing-modes", &prompts_dir));

    fs::write(&dest_path, code).unwrap();

    // Tell Cargo to re-run if any prompt file changes
    println!("cargo:rerun-if-changed=prompts/");
}

fn discover_prompts(dir: &Path) -> Vec<(String, PathBuf)> {
    let mut entries = Vec::new();
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                let name = path.file_stem().unwrap().to_string_lossy().to_string();
                entries.push((name, path));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

fn to_const_name(name: &str) -> String {
    name.replace('-', "_").to_uppercase()
}

fn generate_category(prompts_dir: &Path, category: &str) -> String {
    let dir = prompts_dir.join(category);
    let entries = discover_prompts(&dir);
    let mod_name = category.replace('-', "_");

    let mut code = format!("pub mod {} {{\n", mod_name);
    for (name, path) in &entries {
        let const_name = to_const_name(name);
        let path_str = path.display();
        code.push_str(&format!(
            "    pub const {}: &str = include_str!(\"{}\");\n",
            const_name, path_str
        ));
    }
    code.push_str("}\n\n");
    code
}

fn generate_lookup_fn(fn_suffix: &str, category: &str, prompts_dir: &Path) -> String {
    let dir = prompts_dir.join(category);
    let entries = discover_prompts(&dir);

    let mut code = format!(
        "pub fn embedded_{}(name: &str) -> Option<&'static str> {{\n    match name {{\n",
        fn_suffix
    );
    for (name, _) in &entries {
        let const_name = to_const_name(name);
        let mod_name = category.replace('-', "_");
        code.push_str(&format!(
            "        \"{}\" => Some({}::{}),\n",
            name, mod_name, const_name
        ));
    }
    code.push_str("        _ => None,\n    }\n}\n\n");
    code
}

fn generate_names_fn(fn_suffix: &str, category: &str, prompts_dir: &Path) -> String {
    let dir = prompts_dir.join(category);
    let entries = discover_prompts(&dir);

    let names: Vec<String> = entries.iter().map(|(n, _)| format!("\"{}\"", n)).collect();
    format!(
        "pub fn all_{}_names() -> &'static [&'static str] {{\n    &[{}]\n}}\n\n",
        fn_suffix,
        names.join(", ")
    )
}
```

**Step 4: Update main.rs to include the generated module**

```rust
pub mod generated {
    pub mod prompts {
        include!(concat!(env!("OUT_DIR"), "/prompts.rs"));
    }
}

fn main() {
    println!("agenticenti");
}
```

Also update `Cargo.toml` to set lib + bin:

```toml
[lib]
name = "agenticenti"
path = "src/lib.rs"

[[bin]]
name = "agenticenti"
path = "src/main.rs"
```

Create `src/lib.rs`:

```rust
pub mod generated {
    pub mod prompts {
        include!(concat!(env!("OUT_DIR"), "/prompts.rs"));
    }
}
```

And update `src/main.rs` to use the lib:

```rust
use agenticenti::generated::prompts;

fn main() {
    // Placeholder — will be replaced by clap CLI in next task
    for name in prompts::all_role_names() {
        println!("role: {}", name);
    }
}
```

**Step 5: Run tests to verify they pass**

Run: `cargo test`
Expected: All 5 tests pass

**Step 6: Commit**

```bash
git add build.rs src/lib.rs src/main.rs tests/build_test.rs Cargo.toml
git commit -m "feat: add build.rs for compile-time prompt embedding"
```

---

## Task 8: Implement the composer module

**Files:**
- Create: `src/composer.rs`
- Create: `tests/composer_test.rs`

**Step 1: Write failing tests for the composer**

Create `tests/composer_test.rs`:

```rust
use std::path::PathBuf;
use agenticenti::composer::{compose, resolve_prompt, list_available, PromptCategory};

#[test]
fn test_resolve_embedded_role() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = resolve_prompt(&config_dir, PromptCategory::Role, "coder");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("Role: Coder"));
}

#[test]
fn test_resolve_unknown_role_fails() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = resolve_prompt(&config_dir, PromptCategory::Role, "nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_resolve_override_takes_precedence() {
    // Create a temp dir with an override file
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
    let result = compose("coder", &[], None, &config_dir);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Role: Coder"));
    assert!(!output.contains("---")); // No separator when only one piece
}

#[test]
fn test_compose_role_plus_overlay() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &["rust".to_string()], None, &config_dir);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Role: Coder"));
    assert!(output.contains("Overlay: Rust"));
    assert!(output.contains("\n\n---\n\n")); // Separator between pieces
}

#[test]
fn test_compose_role_plus_multiple_overlays() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("coder", &["rust".to_string(), "docker".to_string()], None, &config_dir);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Role: Coder"));
    assert!(output.contains("Overlay: Rust"));
    assert!(output.contains("Overlay: Docker"));
}

#[test]
fn test_compose_with_testing_mode() {
    let config_dir = PathBuf::from("/nonexistent");
    let result = compose("tester", &["rust".to_string()], Some("unit"), &config_dir);
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Role: Tester"));
    assert!(output.contains("Overlay: Rust"));
    assert!(output.contains("Testing Mode Overlay: Unit"));
}

#[test]
fn test_list_available_includes_all() {
    let config_dir = PathBuf::from("/nonexistent");
    let roles = list_available(PromptCategory::Role, &config_dir);
    assert!(roles.len() >= 15);
    assert!(roles.iter().any(|r| r.name == "coder"));
    assert!(roles.iter().any(|r| r.name == "evangelist"));
    assert!(roles.iter().any(|r| r.name == "github-actions"));
}
```

**Step 2: Add tempfile dev-dependency**

Add to `Cargo.toml`:

```toml
[dev-dependencies]
tempfile = "3"
```

**Step 3: Run tests to verify they fail**

Run: `cargo test`
Expected: FAIL — composer module doesn't exist

**Step 4: Write the composer module**

Create `src/composer.rs`:

```rust
use std::fs;
use std::path::Path;

use crate::generated::prompts;

#[derive(Debug, Clone, Copy)]
pub enum PromptCategory {
    Role,
    Overlay,
    TestingMode,
}

impl PromptCategory {
    fn dir_name(&self) -> &'static str {
        match self {
            PromptCategory::Role => "roles",
            PromptCategory::Overlay => "overlays",
            PromptCategory::TestingMode => "testing-modes",
        }
    }

    fn embedded_lookup(&self, name: &str) -> Option<&'static str> {
        match self {
            PromptCategory::Role => prompts::embedded_role(name),
            PromptCategory::Overlay => prompts::embedded_overlay(name),
            PromptCategory::TestingMode => prompts::embedded_testing_mode(name),
        }
    }

    fn embedded_names(&self) -> &'static [&'static str] {
        match self {
            PromptCategory::Role => prompts::all_role_names(),
            PromptCategory::Overlay => prompts::all_overlay_names(),
            PromptCategory::TestingMode => prompts::all_testing_mode_names(),
        }
    }
}

#[derive(Debug)]
pub struct AvailablePrompt {
    pub name: String,
    pub source: PromptSource,
}

#[derive(Debug, PartialEq)]
pub enum PromptSource {
    Embedded,
    Override,
    UserOnly,
}

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

pub fn compose(
    role: &str,
    languages: &[String],
    testing_mode: Option<&str>,
    config_dir: &Path,
) -> Result<String, ComposeError> {
    let mut parts = vec![resolve_prompt(config_dir, PromptCategory::Role, role)?];

    for lang in languages {
        parts.push(resolve_prompt(config_dir, PromptCategory::Overlay, lang)?);
    }

    if let Some(mode) = testing_mode {
        parts.push(resolve_prompt(config_dir, PromptCategory::TestingMode, mode)?);
    }

    Ok(parts.join("\n\n---\n\n"))
}

pub fn list_available(category: PromptCategory, config_dir: &Path) -> Vec<AvailablePrompt> {
    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

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
```

**Step 5: Update lib.rs to export composer**

```rust
pub mod composer;
pub mod generated {
    pub mod prompts {
        include!(concat!(env!("OUT_DIR"), "/prompts.rs"));
    }
}
```

**Step 6: Run tests to verify they pass**

Run: `cargo test`
Expected: All tests pass

**Step 7: Commit**

```bash
git add src/composer.rs src/lib.rs tests/composer_test.rs Cargo.toml
git commit -m "feat: implement composer module with override resolution"
```

---

## Task 9: Implement the CLI with clap

**Files:**
- Modify: `src/main.rs`
- Create: `tests/cli_test.rs`

**Step 1: Write failing CLI integration tests**

Create `tests/cli_test.rs`:

```rust
use std::process::Command;

fn agenticenti() -> Command {
    Command::new(env!("CARGO_BIN_EXE_agenticenti"))
}

#[test]
fn test_compose_single_role() {
    let output = agenticenti()
        .args(["compose", "coder"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Role: Coder"));
}

#[test]
fn test_compose_role_with_overlay() {
    let output = agenticenti()
        .args(["compose", "coder", "rust"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Role: Coder"));
    assert!(stdout.contains("Overlay: Rust"));
}

#[test]
fn test_compose_with_testing_mode() {
    let output = agenticenti()
        .args(["compose", "tester", "rust", "--testing-mode", "unit"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Role: Tester"));
    assert!(stdout.contains("Testing Mode Overlay: Unit"));
}

#[test]
fn test_compose_unknown_role_fails() {
    let output = agenticenti()
        .args(["compose", "nonexistent"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn test_list_roles() {
    let output = agenticenti()
        .args(["list", "--roles"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("coder"));
    assert!(stdout.contains("evangelist"));
    assert!(stdout.contains("github-actions"));
}

#[test]
fn test_list_overlays() {
    let output = agenticenti()
        .args(["list", "--overlays"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("bash"));
    assert!(stdout.contains("docker"));
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test --test cli_test`
Expected: FAIL — CLI not implemented

**Step 3: Implement the CLI in main.rs**

```rust
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
        } => {
            match composer::compose(
                &role,
                &languages,
                testing_mode.as_deref(),
                &config_dir,
            ) {
                Ok(prompt) => print!("{}", prompt),
                Err(e) => {
                    eprintln!("error: {}", e);
                    process::exit(1);
                }
            }
        }
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
```

**Step 4: Add dirs dependency for home directory**

Add to `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
dirs = "6"
```

**Step 5: Run all tests**

Run: `cargo test`
Expected: All tests pass (build_test, composer_test, cli_test)

**Step 6: Manual smoke test**

Run: `cargo run -- compose coder rust`
Expected: Composed prompt with coder role + rust overlay

Run: `cargo run -- list`
Expected: All roles, overlays, and testing modes listed

**Step 7: Commit**

```bash
git add src/main.rs tests/cli_test.rs Cargo.toml
git commit -m "feat: implement CLI with compose and list subcommands"
```

---

## Task 10: E2E test — full binary workflow

**Files:**
- Create: `tests/e2e_test.rs`

**Step 1: Write E2E tests**

```rust
use std::process::Command;
use tempfile::TempDir;

fn agenticenti() -> Command {
    Command::new(env!("CARGO_BIN_EXE_agenticenti"))
}

#[test]
fn e2e_compose_coder_rust_produces_valid_markdown() {
    let output = agenticenti()
        .args(["compose", "coder", "rust"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Verify structure: role section, separator, overlay section
    let sections: Vec<&str> = stdout.split("\n\n---\n\n").collect();
    assert_eq!(sections.len(), 2, "Expected 2 sections (role + overlay)");
    assert!(sections[0].contains("# Role: Coder"));
    assert!(sections[1].contains("# Overlay: Rust"));
}

#[test]
fn e2e_compose_tester_e2e_python_terraform() {
    let output = agenticenti()
        .args(["compose", "tester", "python", "terraform", "--testing-mode", "e2e"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    let sections: Vec<&str> = stdout.split("\n\n---\n\n").collect();
    assert_eq!(sections.len(), 4, "Expected 4 sections (role + 2 overlays + testing mode)");
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
    assert!(!stdout.contains("autonomous implementation specialist"));
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
    let output = agenticenti()
        .args(["list"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Roles:"));
    assert!(stdout.contains("Overlays:"));
    assert!(stdout.contains("Testing Modes:"));
}
```

**Step 2: Run E2E tests**

Run: `cargo test --test e2e_test`
Expected: All E2E tests pass

**Step 3: Commit**

```bash
git add tests/e2e_test.rs
git commit -m "test: add E2E tests for full binary workflow"
```

---

## Task 11: Final verification and cleanup

**Files:**
- Modify: Various (cleanup only)

**Step 1: Run the full test suite**

Run: `cargo test`
Expected: All tests pass

**Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

**Step 3: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues (fix with `cargo fmt` if needed)

**Step 4: Build release binary**

Run: `cargo build --release`
Expected: Binary at `target/release/agenticenti`

**Step 5: Smoke test the release binary**

Run: `./target/release/agenticenti compose coder rust`
Run: `./target/release/agenticenti compose evangelist bash docker`
Run: `./target/release/agenticenti compose github-actions docker`
Run: `./target/release/agenticenti compose tester --testing-mode e2e rust`
Run: `./target/release/agenticenti list`
Expected: All produce correct output

**Step 6: Verify binary size is reasonable**

Run: `ls -lh target/release/agenticenti`
Expected: Small binary (likely <5MB with embedded prompts)

**Step 7: Final commit**

```bash
git add -A
git commit -m "chore: final cleanup and verification"
```

---

## Dependency Graph

```
Task 1 (scaffold) ──→ Task 2 (extract prompts) ──→ Task 7 (build.rs)
                      Task 3 (evangelist)       ──→ Task 7
                      Task 4 (github-actions)   ──→ Task 7
                      Task 5 (bash overlay)     ──→ Task 7
                      Task 6 (docker overlay)   ──→ Task 7

Task 7 (build.rs) ──→ Task 8 (composer) ──→ Task 9 (CLI) ──→ Task 10 (E2E) ──→ Task 11 (cleanup)
```

## Parallelization Notes

- Tasks 2, 3, 4, 5, 6 can all run in parallel (independent .md file creation)
- Task 7 depends on all prompt files existing
- Tasks 8, 9, 10, 11 are sequential (each builds on the previous)

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| build.rs generates invalid Rust | Medium | High | Generated code is simple string includes — test immediately |
| include_str! paths break across platforms | Medium | Medium | Use Path abstractions, test on Linux |
| Prompt content has unescaped sequences | Low | Low | include_str! handles raw content |
| clap argument parsing edge cases | Low | Low | Integration tests cover main patterns |

## Out of Scope

- Project context injection (third part of the formula — left to caller)
- Prompt versioning or changelogs
- Shell completion generation (can add later with `clap_complete`)
- Remote prompt sync
- TUI for interactive selection
