# Overlay: Go

## Conventions
- Follow `Effective Go` and the Go Code Review Comments wiki
- Use `gofmt` / `goimports` — formatting is non-negotiable
- Exported names are PascalCase, unexported are camelCase
- Package names are lowercase, single-word, no underscores
- One type per file for large types, group small related types

## Error Handling
- Always check errors: `if err != nil { return ..., fmt.Errorf("context: %w", err) }`
- Use `%w` for wrapping (enables `errors.Is` / `errors.As` unwrapping)
- Custom error types implement the `error` interface
- Never ignore errors with `_` unless you add a comment explaining why
- Use sentinel errors (`var ErrNotFound = errors.New("not found")`) for expected errors

## Patterns
- Accept interfaces, return structs
- Use `context.Context` as the first parameter for functions that do I/O
- Use `defer` for cleanup (files, locks, connections) — immediately after acquisition
- Prefer table-driven tests with `t.Run()` subtests
- Use `sync.WaitGroup` for goroutine coordination, channels for communication

## Testing
- Test files are `*_test.go` in the same package
- Use `testing.T` for unit tests, `testing.B` for benchmarks
- `testify/assert` or `testify/require` if the project uses them, otherwise stdlib
- Use `t.Helper()` in test helper functions for correct line reporting
- `t.Parallel()` for tests that can run concurrently

## Dependencies
- Use Go modules (`go.mod` / `go.sum`)
- `go mod tidy` after any dependency changes
- Minimize external dependencies — Go stdlib is extensive
- Vendor dependencies if the project uses vendoring

## Build & Verify
- `go build ./...` — compile
- `go test ./...` — run all tests
- `go vet ./...` — static analysis
- `golangci-lint run` — comprehensive linting
- `go mod tidy` — clean up dependencies

## References

- https://go.dev/doc/
- https://go.dev/doc/effective_go
