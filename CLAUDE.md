# CLAUDE.md

Guidance for AI coding agents working in this repository.

## What this is

`sadikkuzu` is a tiny personal "business card" CLI published on crates.io
(`cargo install sadikkuzu`). It prints Sadık Kuzu's profile links.

- Single binary crate, all code in `src/main.rs`.
- Only dependency: `clap` v4 with the `derive` feature.
- Running with no flags prints a greeting; `--github`, `--linkedin`,
  `--x` (alias `--twitter`), `--all` print links. `-h` / `-V` come from
  clap.

Human-facing process lives in [CONTRIBUTING.md](CONTRIBUTING.md): setup,
dev loop, what gates a PR, commit/branch/changelog conventions, versioning
and the release flow. Read it before committing or releasing. This file only
adds what agents need on top.

## Must-follow rules (details in CONTRIBUTING.md)

- One issue → one branch `<type>/<short-topic>` from `main` → one PR with
  `Closes #N`. Never push to `main`.
- Conventional Commits; `!` / `BREAKING CHANGE:` for user-visible CLI breaks.
- User-visible change → bullet under `## [Unreleased]` in `CHANGELOG.md`.
- `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
  must pass; commit `Cargo.lock` with dependency or version changes.
- Keep the crate dependency-light.

## Toolchain

The main checkout has a project-local Rust toolchain in `./renv/` (see
CONTRIBUTING.md). Activate it before `cargo`:

```shell
. ./renv/bin/activate
```

If `renv/` is missing, plain `cargo` from a system rustup install works too.

## Parallel work with git worktrees

When several agents work on issues at the same time, each one gets its own
worktree (separate directory + branch, shared `.git`). Never switch branches
in a directory another agent is using.

Create a worktree (from the root of the main checkout):

```shell
W=../sadikkuzu.rs.worktrees            # sibling dir, nothing to git-ignore
git fetch origin
git worktree add --no-track $W/11 -b test/cli-integration origin/main
```

Work inside it:

```shell
cd ../sadikkuzu.rs.worktrees/11
# activate the main checkout's renv (located via the shared .git dir)
. "$(dirname "$(git rev-parse --git-common-dir)")/renv/bin/activate"
cargo test
git push -u origin HEAD                # first push; plain `git push` after
```

- `renv/` is git-ignored, so worktrees don't have it: activate the main
  checkout's `renv` as shown above. Don't symlink it into the worktree
  (the symlink would show up as an untracked file).
- If your shell doesn't keep state between commands (e.g. Claude Code's
  Bash tool keeps only the cwd), source `activate` in the same command as
  `cargo`, e.g. `. "<path>/renv/bin/activate" && cargo test`.
- `--no-track` keeps `origin/main` from becoming the branch's upstream;
  otherwise `git push` is rejected and `git status` reports
  "ahead of origin/main".
- pre-commit hooks live in the shared `.git/hooks` (unless `core.hooksPath`
  is set); `pre-commit install` once in the main checkout covers every
  worktree. The cargo hooks use whatever `cargo` is on `PATH` in the shell
  running `git commit`, so activate `renv` there too.
- Each worktree has its own `target/`; the first build there is a full one.
  All worktrees share renv's `CARGO_HOME`, so parallel builds may briefly
  print "Blocking waiting for file lock on package cache".
- Only run independent issues in parallel. Issues that touch the same code
  (e.g. `src/main.rs` refactors) go one after another, each starting from
  a fresh `origin/main` after the previous PR is merged.
- After the PR is merged, from the main checkout (on `main`):
  ```shell
  W=../sadikkuzu.rs.worktrees
  git worktree remove $W/11
  git pull --ff-only --prune           # local main must contain the merge
  git branch -d test/cli-integration   # refused if main is stale
  git worktree prune
  ```

## Releasing

Follow "Versioning and releases" in CONTRIBUTING.md. Pushing a tag starts a
crates.io publish, so tag only when the user asks, after checking the date in
the `## [X.Y.Z]` heading and that `main` CI is green on the merge commit.
