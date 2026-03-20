# Multi-Workspace Support

## Current State

The config schema already supports multiple workspaces via `BTreeMap<String, WorkspaceConfig>`, but `current_workspace()` always returns the first entry alphabetically. Only one workspace is ever used at runtime. There is no workspace-switching UI or state.

## Implementation Plan

### PR 1: Workspace selection state (no UI)

**Goal:** Wire up workspace selection in `App` so the correct workspace is used at runtime.

Changes:
- Add `selected_workspace: String` to `App` (initialized to first key)
- Change `Config::current_workspace()` to accept a `name: &str` param instead of hardcoding `nth(0)`
- Add `App::switch_workspace(name: &str)` to update selected workspace and trigger a post reload
- Pass the full `Config` into `App` so it can look up workspace configs on demand
- Update all call sites in `main.rs` / `app.rs`

This is pure plumbing — no behavior change for single-workspace users.

---

### PR 2: Workspace switcher UI

**Goal:** Let users see and switch between workspaces.

Changes:
- Add a workspace tab bar (or popup/modal) above the post-list tabs
- Keyboard shortcuts: `[` / `]` to cycle through workspaces
- Display current workspace name in the header/border title
- Call `switch_workspace()` (from PR 1) when user selects a workspace
- Apply the new workspace's theme on switch

---

### PR 3: Per-workspace post list state

**Goal:** Each workspace independently remembers its post list, selected view, and pagination.

Changes:
- Move `PostList` state into a `BTreeMap<String, PostList>` in `App` (keyed by workspace name)
- On workspace switch: restore cached `PostList` if available, or create a fresh one and load posts
- Ensure pagination state is isolated per workspace
- Handle per-workspace theme transitions cleanly
