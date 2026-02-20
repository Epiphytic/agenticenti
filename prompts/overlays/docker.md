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

## References

- https://docs.docker.com/build/building/best-practices/ — Docker official build best practices
- https://github.com/hadolint/hadolint — Hadolint rules and Dockerfile linting
