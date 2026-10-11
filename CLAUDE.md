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
cargo clippy -- -D warnings        # must be warning-free
cargo publish --dry-run            # verify the package before a release
pre-commit run --all-files         # run all local pre-commit hooks
```

## Checks that gate changes

- **pre-commit** (`.pre-commit-config.yaml`): end-of-file, trailing
  whitespace, YAML/TOML validity, merge conflicts, `cargo fmt`,
  `cargo check`, `cargo clippy -- -D warnings`. Install with
  `pre-commit install`. pre-commit.ci skips `cargo-check` and `clippy`
  (no network there), so run them locally.
- **GitHub Actions** (`.github/workflows/rust.yml`): `cargo build` and
  `cargo test` on pushes and PRs to `main`.

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
- Keep the crate dependency-light; don't add crates without a clear need.

## Parallel work with git worktrees

When several agents work on issues at the same time, each one gets its own
worktree (separate directory + branch, shared `.git`). Never switch branches
in a directory another agent is using.

```shell
cd ~/code/myrepos/sadikkuzu.rs
git switch main && git pull --ff-only
W=../sadikkuzu.rs.worktrees            # sibling dir, nothing to git-ignore
git worktree add $W/11 -b test/cli-integration origin/main

cd $W/11
. ~/code/myrepos/sadikkuzu.rs/renv/bin/activate   # main checkout's renv
```

- `renv/` is git-ignored, so worktrees don't have it: activate the main
  checkout's `renv` by absolute path. Don't symlink it into the worktree
  (the symlink would show up as an untracked file).
- pre-commit hooks live in the shared `.git/hooks`; `pre-commit install`
  once in the main checkout covers every worktree.
- Each worktree has its own `target/`; the first build there is a full one.
- Only run independent issues in parallel. Issues that touch the same code
  (e.g. `src/main.rs` refactors) go one after another, each starting from
  a fresh `origin/main` after the previous PR is merged.
- After the PR is merged:
  ```shell
  git worktree remove ../sadikkuzu.rs.worktrees/11
  git branch -d test/cli-integration
  git fetch --prune && git worktree prune
  ```

## Releasing (current manual flow)

1. Bump `version` in `Cargo.toml`, run `cargo update -p sadikkuzu`
   so `Cargo.lock` matches, commit as `chore(release): X.Y.Z`.
2. `cargo publish --dry-run`, then tag `X.Y.Z` and push the tag.
3. `cargo publish` (needs `cargo login` with a crates.io token).
4. Create the GitHub Release for the tag.

Check the open issues before changing this flow: there are plans for a
CHANGELOG and an automated publish workflow.
