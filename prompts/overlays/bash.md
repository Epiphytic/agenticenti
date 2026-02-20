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

## References

- https://google.github.io/styleguide/shellguide.html — Google Shell Style Guide
- https://www.shellcheck.net/ — ShellCheck wiki with detailed explanations of each rule
