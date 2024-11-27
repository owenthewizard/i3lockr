use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, io};

use clap::ValueEnum;
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");
//use clap::Shell;

fn main() -> Result<(), io::Error> {
    // Export build target, build time, and git commit
    println!(
        "cargo:rustc-env=TARGET={}",
        env::var("TARGET").unwrap_or_else(|_| "Unknown Target".to_string())
    );

    let mut git_branch = "Unknown Branch".to_string();
    let mut git_commit = "Unknown Commit".to_string();

    if let Ok(out) = Command::new("git")
        .args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        if !out.stdout.is_empty() {
            git_branch =
                String::from_utf8(out.stdout).unwrap_or_else(|_| "Unknown Branch".to_string());
        }
    }

    if let Ok(out) = Command::new("git")
        .args(vec!["rev-parse", "--short", "HEAD"])
        .output()
    {
        if !out.stdout.is_empty() {
            git_commit =
                String::from_utf8(out.stdout).unwrap_or_else(|_| "Unknown Commit".to_string());
        }
    }

    println!("cargo:rustc-env=GIT_BRANCH={git_branch}");
    println!("cargo:rustc-env=GIT_COMMIT={git_commit}");
    println!(
        "cargo:rustc-env=TIME={}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = clap::command!();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, env!("CARGO_PKG_NAME"), outdir.as_os_str())?;
    }

    Ok(())
}
