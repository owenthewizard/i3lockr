use std::process::Command;

fn main() {
    println!("cargo:rustc-env=GIT_BRANCH=unknown-branch");
    println!("cargo:rustc-env=GIT_HASH=unknown-commit");
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        if output.status.success() {
            let git_branch = String::from_utf8(output.stdout).unwrap();
            println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
        }
    }
    if let Ok(output) = Command::new("git").args(&["rev-parse", "HEAD"]).output() {
        if output.status.success() {
            let git_hash = String::from_utf8(output.stdout).unwrap();
            println!("cargo:rustc-env=GIT_HASH={}", git_hash);
        }
    }

    // TODO: clap gen shell completion & manpages
}
