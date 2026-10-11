# CLAUDE.md

Guidance for AI coding agents working in this repository.

## What this is

`sadikkuzu` is a tiny personal "business card" CLI published on crates.io
(`cargo install sadikkuzu`). It prints Sadık Kuzu's profile links.

- Single binary crate, all code in `src/main.rs`.
- Only dependency: `clap` v4 with the `derive` feature.
- Running with no flags prints a greeting; `--github`, `--linkedin`,
  `--twitter`, `--all` print links. `-h` / `-V` come from clap.

## Toolchain

A project-local Rust toolchain lives in `./renv/` (created with
[rustenv](https://github.com/chriskuehl/rustenv); it git-ignores itself).

```shell
. ./renv/bin/activate      # puts renv's cargo/rustc/rustup first on PATH
rustup update stable       # updates only the toolchain inside renv/
deactivate_rustenv
```

If `renv/` is missing, plain `cargo` from a system rustup install works too.

## Common commands

```shell
cargo run -- --all                 # run locally
cargo build
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings  # must be warning-free
cargo publish --dry-run            # verify the package before a release
pre-commit run --all-files         # run all local pre-commit hooks
```

## Checks that gate changes

- **pre-commit** (`.pre-commit-config.yaml`): end-of-file, trailing
  whitespace, YAML/TOML validity, merge conflicts, `cargo fmt`,
  `cargo check`, `cargo clippy --all-targets -- -D warnings`. Install
  with `pre-commit install`. pre-commit.ci skips `cargo-check` and
  `clippy` (no network there); GitHub Actions covers clippy instead.
- **GitHub Actions** (`.github/workflows/rust.yml`): on pushes and PRs
  to `main`, runs `cargo fmt --all --check`,
  `cargo clippy --all-targets --locked -- -D warnings`,
  `cargo test --locked` and `cargo publish --dry-run --locked`.
  `--locked` fails if `Cargo.lock` is out of date, so commit it with
  any dependency or version change.

## Conventions

- **Commits:** [Conventional Commits](https://www.conventionalcommits.org/)
  (`feat:`, `fix:`, `docs:`, `ci:`, `chore:`, `refactor:`, `test:`,
  `build:`). Use `!` / `BREAKING CHANGE:` for user-visible CLI breaks.
- **Branches & PRs:** one branch and one PR per issue, branched from
  `main`, named `<type>/<short-topic>` (e.g. `ci/release-workflow`).
  Reference the issue in the PR body (`Closes #N`). Do not push to `main`
  directly.
- **Versioning:** SemVer. While on `0.x`, breaking CLI changes bump the
  minor version.
- **Tags:** plain `X.Y.Z` (no `v` prefix), matching existing tags
  `0.1.0` … `0.2.0`.
- **Changelog:** `CHANGELOG.md` follows
  [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/). Every PR
  with a user-visible change adds a bullet under `## [Unreleased]`, in the
  matching `### Added` / `Changed` / `Deprecated` / `Removed` / `Fixed` /
  `Security` group (create the group if missing), ending with the PR or
  issue number, e.g. `(#14)`. Pure CI, tooling or docs-for-contributors
  changes don't need an entry.
- Keep the crate dependency-light; don't add crates without a clear need.

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

## Releasing (current manual flow)

1. Bump `version` in `Cargo.toml`, run `cargo update -p sadikkuzu`
   so `Cargo.lock` matches.
2. In `CHANGELOG.md`, rename `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`,
   add a fresh empty `## [Unreleased]` above it, and update the links at the
   bottom: `[Unreleased]` compares `X.Y.Z...HEAD`, the new `[X.Y.Z]`
   compares `<previous>...X.Y.Z`.
3. Commit as `chore(release): X.Y.Z`.
4. `cargo publish --dry-run`, then tag `X.Y.Z` and push the tag.
5. `cargo publish` (needs `cargo login` with a crates.io token).
6. Create the GitHub Release for the tag, using the `## [X.Y.Z]` section
   of `CHANGELOG.md` as the release notes.

Check the open issues before changing this flow: there are plans for an
automated publish workflow.
