use assert_cmd::Command;
use predicates::prelude::*;

const GREETING: &str = "Aloha, World! This is SADIK KUZU.\n";
const GITHUB: &str = "https://github.com/sadikkuzu\n";
const LINKEDIN: &str = "https://linkedin.com/in/sadikkuzu\n";
const TWITTER: &str = "https://twitter.com/sadikkuzu_mba\n";

fn cmd() -> Command {
    Command::cargo_bin("sadikkuzu").unwrap()
}

fn all_links() -> String {
    [GITHUB, LINKEDIN, TWITTER].concat()
}

#[test]
fn no_args_prints_greeting() {
    cmd().assert().success().stdout(GREETING);
}

#[test]
fn github_prints_github_link() {
    cmd().arg("--github").assert().success().stdout(GITHUB);
}

#[test]
fn linkedin_prints_linkedin_link() {
    cmd().arg("--linkedin").assert().success().stdout(LINKEDIN);
}

#[test]
fn twitter_prints_twitter_link() {
    cmd().arg("--twitter").assert().success().stdout(TWITTER);
}

#[test]
fn all_prints_every_link_in_order() {
    cmd().arg("--all").assert().success().stdout(all_links());
}

#[test]
fn all_wins_over_single_flags() {
    cmd()
        .args(["--all", "--github"])
        .assert()
        .success()
        .stdout(all_links());
}

#[test]
fn multiple_flags_print_in_fixed_order() {
    cmd()
        .args(["--github", "--twitter"])
        .assert()
        .success()
        .stdout([GITHUB, TWITTER].concat());
}

#[test]
fn repeated_flag_prints_link_once() {
    cmd()
        .args(["--github", "--github"])
        .assert()
        .success()
        .stdout(GITHUB);
}

#[test]
fn many_repeated_flags_print_links_once() {
    // 800 flag occurrences must neither panic nor change the output.
    let args = ["--github", "--linkedin", "--twitter", "--all"].repeat(200);
    cmd().args(args).assert().success().stdout(all_links());
}

#[test]
fn version_flag_prints_version() {
    cmd().arg("-V").assert().success().stdout(concat!(
        "sadikkuzu ",
        env!("CARGO_PKG_VERSION"),
        "\n"
    ));
}

#[test]
fn help_flag_lists_options() {
    cmd().arg("-h").assert().success().stdout(
        predicate::str::contains("--github")
            .and(predicate::str::contains("--linkedin"))
            .and(predicate::str::contains("--twitter"))
            .and(predicate::str::contains("--all"))
            // Flags are on/off switches, not counters (no `--github...`).
            .and(predicate::str::contains("...").not()),
    );
}

#[test]
fn unknown_flag_fails() {
    cmd()
        .arg("--nope")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("unexpected argument"));
}
