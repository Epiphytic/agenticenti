# ADR-001: WASM/npm Distribution via WASI Preview 1

## Status

Accepted

## Context

agenticenti is a Rust CLI that composes agent prompts. To distribute it via npm
so users can run `npx @epiphytic/agenticenti compose coder rust` without
installing the Rust toolchain, we need a portable binary format.

Options considered:

1. **Native binaries per platform** — requires CI matrix for linux/mac/windows,
   large download sizes, complex install scripts.
2. **WebAssembly (WASI Preview 1)** — single `.wasm` artifact, runs on any
   Node.js 18+ with the built-in `node:wasi` module.
3. **Rewrite in JavaScript/TypeScript** — defeats the purpose of maintaining a
   single Rust codebase.

## Decision

Compile to `wasm32-wasip1` and ship via npm with a Node.js WASI runner.

### Key trade-offs

- **Removed `dirs` crate**: The `dirs` crate uses platform-specific syscalls
  (e.g., `getpwuid_r`) that are unavailable in WASI. Replaced with
  `std::env::var("HOME")` which works on both native and WASI targets. This
  drops Windows `%USERPROFILE%` support, which is acceptable since the primary
  use case is Unix-like environments and CI.

- **Test gating**: Integration and E2E tests that spawn subprocesses
  (`std::process::Command`) are gated with `#![cfg(not(target_family = "wasm"))]`
  since WASI does not support process spawning.

- **Filesystem preopens**: The Node.js runner preopens `/` -> `/` to give the
  WASM module full filesystem access, which is required for reading config files
  from `$HOME/.agenticenti` and user-specified `--config-dir` paths.

## Consequences

- Single artifact (~1-2 MB `.wasm`) instead of per-platform binaries.
- No native code needed on the consumer side — just `node >= 18`.
- `node:wasi` is marked as experimental in Node.js but has been stable in
  practice since Node 18.
- Future: if WASI Preview 2 component model matures, we can migrate for better
  capability control.
