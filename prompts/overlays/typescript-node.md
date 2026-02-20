# Overlay: TypeScript / Node.js

## Conventions
- Strict TypeScript: `strict: true` in tsconfig, no `any` (use `unknown` if type is truly unknown)
- Use `const` by default, `let` only when reassignment is needed, never `var`
- Named exports over default exports (better refactoring support)
- Use `import type { ... }` for type-only imports
- ESM modules (`import/export`) unless the project is explicitly CJS

## Package Management
- Use the lockfile that exists (package-lock.json → npm, yarn.lock → yarn, pnpm-lock.yaml → pnpm)
- Never switch package managers mid-project
- Use `--save-exact` for application dependencies
- Run `npm audit` / `pnpm audit` before adding new dependencies

## Error Handling
- Use typed errors (custom Error subclasses) not string throws
- Always handle Promise rejections — no unhandled promise warnings
- Use try/catch at system boundaries (API handlers, CLI entry points)
- For expected failures, use Result types (`{ok: true, data: T} | {ok: false, error: E}`)
  or a library like `neverthrow`

## Testing
- Use the test framework already in the project (Jest, Vitest, or Node test runner)
- `describe` for grouping, `it` for individual cases
- Use `beforeEach`/`afterEach` for setup/teardown, not `beforeAll` (isolate tests)
- Mock at module boundaries, not internal functions
- For async tests, always `await` assertions or return the promise

## Async Patterns
- `async/await` over raw Promises over callbacks
- Use `Promise.all()` for concurrent independent operations
- Use `Promise.allSettled()` when you need results even if some fail
- AbortController for cancellation — pass signals through the call chain
- Avoid `setTimeout` for coordination — use proper event-based patterns

## Build & Verify
- `tsc --noEmit` — type checking
- `npm test` / `vitest run` — run tests
- `eslint .` — lint
- `prettier --check .` — formatting

## References

- https://www.typescriptlang.org/docs/
- https://nodejs.org/en/docs
