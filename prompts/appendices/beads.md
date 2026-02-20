# Appendix: Beads Issue Tracking

The [beads](https://github.com/steveyegge/beads) issue tracker is initialized in this
repository (`.beads/` directory). Beads issues are git-committed and survive across
sessions, branches, and machines — unlike ephemeral built-in tasks.

## Quick Reference

| Action | Command |
|--------|---------|
| List open issues | `bd list --status open` |
| Find unblocked work | `bd ready` |
| Show issue details | `bd show <id>` |
| Create issue | `bd create --title "..." --type task\|feature\|bug` |
| Start work | `bd update <id> --status in_progress` |
| Add progress comment | `bd comments add <id> "your update"` |
| Close issue | `bd close <id> --comment "Done. Artifacts: ..."` |

## Update Protocol

Update beads issues at these checkpoints:

| When | Action | Command |
|------|--------|---------|
| Starting work on an issue | Set status to in_progress | `bd update <id> --status in_progress` |
| Reaching a milestone | Add a progress comment | `bd comments add <id> "Completed X, starting Y"` |
| Making a key decision | Record the rationale | `bd comments add <id> "Chose JWT over sessions because..."` |
| Discovering follow-up work | Create a new issue | `bd create --title "..." --type task` |
| Completing the work | Close with summary | `bd close <id> --comment "Done. Artifacts: ..."` |
| Hitting a blocker | Record the blocker | `bd comments add <id> "Blocked on: ..."` |
| Session ending before completion | Leave a handoff comment | `bd comments add <id> "Progress: X done, Y remaining. Next steps: ..."` |

## Handoff Comments

The handoff comment is critical — it makes multi-session work possible. When a new
session starts, the next agent reads `bd show <id>` and picks up exactly where the
previous session left off, with full decision history.

## Two-Layer Model

Use beads and built-in tasks together:

- **Beads** = durable source of truth (project roadmap, epics, multi-session work)
- **Built-in tasks** = real-time coordination surface (in-session agent work)

Create ephemeral built-in tasks from beads issues for in-session coordination. When
agents complete their work, close the beads issue. For work that can't finish this
session, add a handoff comment and leave the issue open.
