# Branch Protection

Configure branch protection for `main` to gate merges on reviews and CI.

1) Repository settings → Branches → Add rule

- Branch name pattern: `main`
- Require a pull request before merging: enabled (min 1 approving review)
- Dismiss stale approvals when new commits are pushed: enabled
- Require status checks to pass before merging: enabled
  - Select checks:
    - `CI` (workflow running fmt, clippy, tests)
    - `Security Check` (workflow validating tag immutability)
- Require branches to be up to date before merging: enabled
- Include administrators: enabled (recommended)
- Restrict who can push to matching branches: enabled (optional; maintainers only)
- Require signed commits: enabled (optional, if your org enforces it)
- Do not allow force pushes and deletions: enabled

Notes

- Status check names correspond to workflow names in `.github/workflows/ci.yml` and `.github/workflows/security.yml`.
- After saving, GitHub will surface these required checks on each PR into `main`.
- Combine with CODEOWNERS if you want specific reviewers to be auto‑required.

