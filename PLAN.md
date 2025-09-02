# Planning

This file mirrors the assistant's live plan so it persists across sessions.

How it works
- Source of truth: `.codex/plan.json` (machine‑readable).
- Human view: this `PLAN.md` (concise summary and notes).
- Convenience: `make plan` prints both for a quick glance.

Current Plan

1. Inventory dead/duplicate code — in progress
2. DRY refactors across crates — pending
3. Optimize hotspots (profiles/allocs) — pending
4. Strengthen `mimic_stats` accuracy/edges — pending
5. Expand tests for queries/indexes — pending
6. CI: clippy, fmt, all tests — pending
7. Document changes and upgrade notes — pending

Notes
- Feel free to edit `PLAN.md` by hand for extra commentary.
- The assistant will keep `.codex/plan.json` in sync when updating the plan tool.
- For long-running tasks, we can snapshot milestones here with dates.
