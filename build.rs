use std::{env, fs, io};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<(), io::Error> {
    let outdir = "shell-completions";
    fs::create_dir_all(outdir)?;

    let mut cmd = Cli::command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, env!("CARGO_PKG_NAME"), outdir)?;
    }

    Ok(())
}
