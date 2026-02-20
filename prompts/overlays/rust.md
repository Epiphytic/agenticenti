# Overlay: Rust

## Conventions
- Use `thiserror` for library error types, `anyhow` for application error types
- Prefer `Result<T, E>` over panics — `unwrap()` is only acceptable in tests and
  infallible cases (with a comment explaining why it can't fail)
- Use `tracing` for structured logging, not `println!` or `log`
- Use `clippy` as a strict linter: `cargo clippy -- -D warnings`
- Format with `cargo fmt` — no style arguments
- Derive traits in consistent order: `Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize`

## Async Patterns
- Use `tokio` as the async runtime (unless the project uses `async-std`)
- Use `std::sync::Mutex` in `spawn_blocking` contexts, `tokio::sync::Mutex` in async contexts
- Acquire `tokio::sync::Semaphore` permits in async context before moving into `spawn_blocking`
- Prefer `tokio::select!` for concurrent operations over manual polling
- Be careful with `Arc<Mutex<T>>`: prefer channels (`tokio::sync::mpsc`) for cross-task communication

## Error Handling
- Every public function returns `Result<T, ModuleError>` where `ModuleError` is defined
  with `thiserror` in the module
- Use `?` for propagation, `.context("...")` (from anyhow) or `.map_err(...)` for enrichment
- Never use `.unwrap()` in async code — it crashes the runtime

## Testing
- Unit tests go in `#[cfg(test)] mod tests { ... }` at the bottom of the source file
- Integration tests go in `tests/` directory
- Use `#[tokio::test]` for async tests
- Use `tempfile::TempDir` for filesystem tests
- Use `assert_matches!` for enum variant assertions

## Dependencies
- Pin exact versions in `Cargo.toml` for applications (not libraries)
- Run `cargo audit` to check for known vulnerabilities
- Minimize dependency count — Rust compile times scale with dependencies

## Build & Verify
- `cargo build` — check compilation
- `cargo test` — run all tests
- `cargo clippy -- -D warnings` — lint with zero warnings
- `cargo fmt --check` — verify formatting
- `cargo doc --no-deps` — verify documentation builds

## References

- https://doc.rust-lang.org/stable/
- https://rust-lang.github.io/api-guidelines/
