# Contributing

Thanks for helping out. `sadikkuzu` is a tiny CLI, so the process is
small too: one issue, one branch, one pull request.

## Prerequisites

- Rust via [rustup](https://rustup.rs). The minimum supported Rust version
  (MSRV) is `rust-version` in `Cargo.toml` (currently 1.85); day-to-day
  work uses stable.
- [pre-commit](https://pre-commit.com). With [uv](https://docs.astral.sh/uv/):
  `uv tool install pre-commit` (a persistent install; `uvx pre-commit install`
  would point the git hook at a temporary environment).
- Optional: uv, for the project-local toolchain below.

## Setup

```shell
git clone https://github.com/sadikkuzu/sadikkuzu.rs.git
cd sadikkuzu.rs
pre-commit install
```

## Optional: project-local toolchain

[rustenv](https://github.com/chriskuehl/rustenv) creates a self-contained
Rust install in `./renv/` (it git-ignores itself), so updating it doesn't
touch your global toolchain. Run this from the repo root:

```shell
uvx rustenv renv
. ./renv/bin/activate      # renv's cargo/rustc/rustup first on PATH
rustup update stable       # updates only the toolchain inside renv/
deactivate_rustenv
```

Without `renv/`, a normal rustup install works the same way.

## Development loop

```shell
cargo run -- --all                          # run locally
cargo fmt                                   # format
cargo clippy --all-targets -- -D warnings   # must be warning-free
cargo test                                  # unit + CLI integration tests
pre-commit run --all-files                  # every local hook
```

Optional MSRV check, the same as the `msrv` CI job:

```shell
rustup toolchain install 1.85 --profile minimal
cargo +1.85 test --locked
```

### What gates a pull request

- **pre-commit** (`.pre-commit-config.yaml`): end-of-file, trailing
  whitespace, YAML/TOML validity, merge conflicts, `cargo fmt`,
  `cargo check`, `cargo clippy --all-targets -- -D warnings`.
  pre-commit.ci runs these on every PR but skips `cargo-check` and
  `clippy` (no network there).
- **GitHub Actions** (`.github/workflows/rust.yml`): the `check` job runs
  `cargo fmt --all --check`,
  `cargo clippy --all-targets --locked -- -D warnings`,
  `cargo test --locked` and `cargo publish --dry-run --locked`. The `msrv`
  job runs `cargo test --locked` on the MSRV. When the MSRV changes, update
  `rust-version` in `Cargo.toml`, the job's `toolchain:` and the `1.85`
  mentions in this file together.
- `--locked` fails if `Cargo.lock` is out of date, so commit `Cargo.lock`
  with any dependency or version change.

## Coding standards

- `cargo fmt` and `cargo clippy --all-targets -- -D warnings` clean.
- Behaviour changes come with tests in `tests/cli.rs`.
- Keep the crate dependency-light. Don't add a crate without a clear need.

## Commit messages

[Conventional Commits](https://www.conventionalcommits.org/): `<type>: <summary>`,
optionally with a scope, e.g. `feat(cli): ...`.

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `perf`, `build`, `ci`,
`chore`. Mark user-visible CLI breaks with `!` or a `BREAKING CHANGE:`
footer.

```text
feat(cli): add --x flag and print x.com link
fix: don't panic on many repeated flags
refactor(cli)!: remove the --twitter flag
```

## Branches and pull requests

- One issue → one branch → one PR, branched from `main`.
- Branch name `<type>/<short-topic>`, e.g. `ci/release-workflow`.
- PR title in Conventional Commit form; reference the issue in the body
  (`Closes #N`).
- CI must be green. Don't push to `main` directly.
- Every PR with a user-visible change adds a line to `CHANGELOG.md` (below).

## Changelog

`CHANGELOG.md` follows
[Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/). Add a bullet
under `## [Unreleased]`, in the matching `### Added` / `Changed` /
`Deprecated` / `Removed` / `Fixed` / `Security` group (create the group if
missing), ending with the PR or issue number, e.g. `(#14)`. Pure CI, tooling
or contributor-docs changes don't need an entry.

## Versioning and releases

- [SemVer](https://semver.org). While on `0.x`, breaking CLI changes bump
  the minor version.
- Tags are plain `X.Y.Z`, no `v` prefix.

Pushing a tag `X.Y.Z` runs `.github/workflows/release.yml`: it checks the
tagged commit is on `main` and the tag matches `Cargo.toml`, runs the tests,
publishes to crates.io via Trusted Publishing (GitHub OIDC, no stored token)
and creates the GitHub Release with the `## [X.Y.Z]` section of
`CHANGELOG.md` as notes. The job runs in the `release` environment, which
needs a maintainer's approval. If only the GitHub Release step failed,
"Re-run job" is safe: publishing is skipped when the version is already on
crates.io.

1. On a branch `chore/release-X.Y.Z`: bump `version` in `Cargo.toml`, run
   `cargo update -p sadikkuzu` so `Cargo.lock` matches.
2. In `CHANGELOG.md`, rename `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`,
   add a fresh empty `## [Unreleased]` above it, and update the links at the
   bottom: `[Unreleased]` compares `X.Y.Z...HEAD`, the new `[X.Y.Z]`
   compares `<previous>...X.Y.Z`.
3. Commit as `chore(release): X.Y.Z`, open a PR, merge it.
4. Check the date in the `## [X.Y.Z]` heading is still today; if it isn't,
   fix it in a follow-up PR first. Then tag the merge commit of the last
   release PR (the date fix, if any), not whatever `main` is now, and push
   the tag:
   ```shell
   git fetch origin
   sha=$(gh pr view <PR> --json mergeCommit --jq .mergeCommit.oid)
   git tag X.Y.Z "$sha" && git push origin X.Y.Z
   ```
5. Approve the `release` deployment in the Actions run.

Manual fallback (workflow unavailable; needs `cargo login` with a crates.io
token). Pushing the tag still triggers `release.yml`: reject or cancel that
run's pending `release` deployment so it can't race the manual publish.
`cargo publish` packages the working tree, so check out the merge commit
from step 4 first (the release PR's, or the date fix's if there was one):

```shell
git fetch origin
sha=$(gh pr view <PR> --json mergeCommit --jq .mergeCommit.oid)
git switch --detach "$sha"
cargo publish --dry-run --locked
git tag X.Y.Z "$sha" && git push origin X.Y.Z   # then reject the deployment
cargo publish --locked
awk -v v=X.Y.Z 'index($0, "## [" v "]") == 1 {f=1; next} /^## \[/ {f=0} f' CHANGELOG.md > notes.md
if grep -q '[^[:space:]]' notes.md; then
  gh release create X.Y.Z --title X.Y.Z --verify-tag --notes-file notes.md
else
  echo "no ## [X.Y.Z] section in CHANGELOG.md: GitHub Release not created"
fi
rm notes.md
git switch -                                    # back to where you started
```

## Reporting issues and security

- Bugs and ideas: open a
  [GitHub issue](https://github.com/sadikkuzu/sadikkuzu.rs/issues).
- Vulnerabilities: don't open a public issue. Use
  [private vulnerability reporting](https://github.com/sadikkuzu/sadikkuzu.rs/security/advisories/new)
  instead.
