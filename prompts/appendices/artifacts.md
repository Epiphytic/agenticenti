# Appendix: Artifact Directory Structure

All agent roles coordinate through file artifacts in the `docs/` directory. This creates
a natural bus with full observability — each artifact is a checkpoint that survives agent
restarts and session boundaries.

## Directory Layout

```
docs/
  research/          ← researcher writes, architect + planner read
  architecture/      ← architect writes, planner + coder read
  adr/               ← architect writes, all roles read
  plans/             ← planner writes, coder + tester + devops read
  reviews/           ← reviewer writes, coder + maintainer read
    security/        ← security-reviewer writes, coder + maintainer + devops read
```

## Naming Convention

Files use date-prefixed names: `YYYY-MM-DD-subject.md`

Examples:
- `docs/research/2026-02-08-auth-options.md`
- `docs/plans/2026-02-08-auth-implementation.md`
- `docs/reviews/2026-02-08-auth-review.md`
- `docs/adr/2026-02-08-jwt-over-sessions.md`

## Coordination Rules

1. **Read artifacts before starting work.** Check `docs/` for existing plans, research,
   architecture docs, and reviews relevant to your task. Don't re-derive what's already
   been decided.

2. **Write artifacts when producing knowledge.** If your work produces decisions,
   findings, or designs that other roles will need, write them to the appropriate
   `docs/` subdirectory.

3. **Reference artifact paths, don't duplicate content.** When communicating with
   teammates or updating issues, point to file paths rather than restating content.

4. **Artifacts are checkpoints.** If an agent fails or is restarted, it picks up from
   the last written artifact rather than starting from scratch.
