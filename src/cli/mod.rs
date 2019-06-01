use std::ffi::OsString;
use std::path::PathBuf;

use structopt::StructOpt;

mod validators;

/// Distort a screenshot and run i3lock
// Needs to be fixed upstream in StructOpt
// TODO: checked if my PR is merged
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Prints version information
    #[structopt(short = "V", long = "version", alias = "vers")]
    pub version: bool,

    /// Print how long each step takes, among other things.
    /// Always enabled in debug builds.
    #[structopt(short = "v", long = "verbose", alias = "verb")]
    pub debug: bool,

    /// Blur strength. Example: 10
    #[structopt(
        short = "b",
        long = "blur",
        raw(validator = "validators::greater_than(0)"),
        alias = "rad"
    )]
    pub radius: Option<u8>,

    /// Don't overlay an icon on these monitors. Must be comma separated.
    /// Example: 0,2
    #[structopt(
        long = "ignore-monitors",
        value_name = "0,2",
        require_delimiter = true,
        visible_alias = "ignore"
    )]
    pub ignore: Vec<usize>,

    /// Interpret the icon as a mask, inverting masked pixels
    /// on the screenshot. Try it to see an example.
    #[structopt(long = "invert")]
    pub invert: bool,

    /// Icon placement, "x,y" (from top-left), or "-x,-y" (from bottom-right).
    /// Has no effect without --icon. Must be comma separated. Defaults to center if not specified.
    /// Example: "945,-20"
    #[structopt(
        short = "u",
        long = "position",
        allow_hyphen_values = true,
        value_name = "945,-20",
        number_of_values = 2,
        require_delimiter = true,
        visible_alias = "pos"
    )]
    pub pos: Vec<isize>,

    /// Path to icon to overlay on screenshot.
    #[structopt(
        short = "i",
        long = "icon",
        value_name = "file.png",
        parse(from_os_str)
    )]
    pub path: Option<PathBuf>,

    /// Arguments to pass to i3lock. Example: "--nofork --ignore-empty-password"
    #[structopt(
        value_name = "i3lock",
        takes_value = true,
        multiple = true,
        parse(from_os_str),
        last = true
    )]
    pub i3lock: Vec<OsString>,
}
