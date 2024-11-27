use std::num::{NonZeroU8, NonZeroUsize};
use std::path::PathBuf;

use clap::{ArgAction, Parser};

/// Distort a screenshot and run i3lock
#[derive(Parser, Debug)]
pub struct Cli {
    /// Prints version information
    #[arg(short = 'V', long = "version", alias = "vers")]
    pub version: bool,

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
        value_name = "945,-20",
        num_args = 1,
        value_delimiter = ',',
        allow_negative_numbers = true,
        visible_alias = "pos"
    )]
    pub pos: Vec<isize>,

    /// Path to icon to overlay on screenshot.
    #[arg(
        short = 'i',
        long = "icon",
        value_name = "file.png",
    )]
    pub path: Option<PathBuf>,

    /// Arguments to pass to i3lock. Example: "--nofork --ignore-empty-password"
    #[arg(
        value_name = "i3lock",
        action = ArgAction::Append,
        last = true
    )]
    pub i3lock: Vec<String>,
}
