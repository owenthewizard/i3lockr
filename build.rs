use std::env;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

include!("src/cli/mod.rs");
use structopt::clap::Shell;

fn main() {
    // Build C code for stackblur and statically link
    let c_src = Path::new("src").join("C");
    cc::Build::new()
        .file(c_src.join("stackblur.c"))
        .include(c_src)
        .compile("stackblur");
    //

    // Export build target, build time, and git commit
    println!(
        "cargo:rustc-env=TARGET={}",
        env::var("TARGET").unwrap_or_else(|_| "Unknown Target".to_owned())
    );

    let mut git_branch = "Unknown Branch".to_owned();
    let mut git_commit = "Unknown Commit".to_owned();

    if let Ok(out) = Command::new("git")
        .args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        if !out.stdout.is_empty() {
            git_branch =
                String::from_utf8(out.stdout).unwrap_or_else(|_| "Unknown Branch".to_owned());
        }
    }

    if let Ok(out) = Command::new("git")
        .args(vec!["rev-parse", "--short", "HEAD"])
        .output()
    {
        if !out.stdout.is_empty() {
            git_commit =
                String::from_utf8(out.stdout).unwrap_or_else(|_| "Unknown Commit".to_owned());
        }
    }

    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
    println!(
        "cargo:rustc-env=TIME={}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    // shell completions
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Bash, "target");
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Fish, "target");
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Zsh, "target");
    Cli::clap().gen_completions(env!("CARGO_PKG_NAME"), Shell::Elvish, "target");
}
