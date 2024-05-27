use std::ffi::OsString;
use std::num::{NonZeroU8, NonZeroUsize};
use std::path::PathBuf;

use clap::{Parser, ValueHint};

fn parse_coords(arg: &str) -> Result<(isize, isize), String> {
    let v = arg
        .split(',')
        .map(|x| x.parse::<isize>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<isize>, _>>()?;
    if v.len() != 2 {
        return Err("pos must be two integers".to_owned());
    }
    Ok((v[0], v[1]))
}

/// Distort a screenshot and run i3lock
#[derive(Parser, Debug)]
#[command(version, author)]
pub struct Cli {
    /// Print how long each step takes, among other things.
    /// Always enabled in debug builds.
    #[arg(short = 'v', long = "verbose", alias = "verb", alias = "debug")]
    pub verbose: bool,

    /// Darken the screenshot by [1, 255]. Example: 15
    #[arg(long = "darken", visible_alias = "dark", conflicts_with = "bright")]
    pub dark: Option<NonZeroU8>,

    /// Brighten the screenshot by [1, 255]. Example: 15
    #[arg(long = "brighten", visible_alias = "bright")]
    pub bright: Option<NonZeroU8>,

    /// Blur strength. Example: 10
    #[arg(short = 'b', long = "blur", alias = "rad")]
    pub radius: Option<NonZeroUsize>,

    /// Scale factor. Increases blur strength by a factor of this. Example: 2
    #[arg(short = 'p', long = "scale")]
    pub factor: Option<NonZeroUsize>,

    /// Don't overlay an icon on these monitors. Useful if you're mirroring displays. Must be comma separated.
    /// Example: 0,2
    #[arg(
        long = "ignore-monitors",
        value_name = "0,2",
        value_delimiter = ',',
        visible_alias = "ignore"
    )]
    pub ignore: Vec<usize>,

    /// Interpret the icon as a mask, inverting masked pixels
    /// on the screenshot. Try it to see an example.
    #[arg(long = "invert")]
    pub invert: bool,

    /// Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right).
    /// Has no effect without --icon. Must be comma separated. Defaults to center if not specified.
    /// Example: "945,-20"
    #[arg(
        short = 'u',
        long = "position",
        allow_hyphen_values = true,
        value_name = "x,y",
        value_parser = parse_coords,
        visible_alias = "pos",
    )]
    pub pos: Option<(isize, isize)>,

    /// Path to icon to overlay on screenshot.
    #[arg(
        short = 'i',
        long = "icon",
        value_name = "lock.png",
        value_hint = ValueHint::FilePath,
    )]
    pub path: Option<PathBuf>,

    /// Arguments to pass to i3lock. Example: "--nofork --ignore-empty-password"
    #[arg(value_name = "i3lock", last = true)]
    pub i3lock: Vec<OsString>,
}
