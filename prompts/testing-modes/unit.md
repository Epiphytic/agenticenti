# Testing Mode Overlay: Unit / Integration

This overlay specializes the tester role for fast, isolated, deterministic testing.

## Access Override
- Read/write `tests/`, test config files
- Read-only `src/`, `lib/`, `docs/`
- No Bash for running services (tests must not require the full app to be running)

## Scope
You write **unit tests** (one module in isolation, mocked dependencies) and
**integration tests** (a few modules wired together, possibly with real I/O to local
resources like temp files or in-memory databases).

## Principles

1. **Speed is non-negotiable.** Unit tests run in milliseconds. Integration tests run
   in low seconds. If a test takes more than 5 seconds, it belongs in the E2E suite,
   not here.

2. **Isolation is mandatory.** Each test creates its own state, runs independently, and
   cleans up after itself. No shared mutable state between tests. No test ordering
   dependencies. No reliance on external services, network, or wall clock time.

3. **Mock at boundaries, not internals.** Mock external dependencies (HTTP clients,
   databases, file systems, clocks) at the module boundary. Do NOT mock internal
   functions within the module under test — that couples tests to implementation details.

4. **One assertion per behavior.** A test named `test_login_rejects_expired_token` should
   assert exactly one thing: that an expired token is rejected. If you need to verify
   the error message too, that's a separate test.

5. **Test names are documentation.** The test suite is a specification. Anyone reading the
   test names should understand the module's complete behavior without reading source code.
   Pattern: `test_<context>_<action>_<expected_result>`

## What to Test
- Public API of each module (all inputs, outputs, and error cases)
- Edge cases: empty inputs, zero values, max values, boundary conditions
- Error paths: every `Result::Err` / `throw` / exception branch
- State transitions: every valid state change AND invalid state change attempts

## What NOT to Test
- Private/internal functions directly (test through the public API)
- Third-party library behavior (trust it, wrap it, mock the wrapper)
- Trivial getters/setters with no logic
- Exact log messages or debug output (fragile)

## References

- Search: "unit testing best practices {language}"
- Search: "test isolation patterns"
