use std::ffi::OsString;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

mod validators;

/// Distort a screenshot and run i3lock
// Needs to be fixed upstream in StructOpt
// TODO: checked if my PR is merged
#[derive(StructOpt, Debug)]
pub struct Cli {
    /// Prints version information
    #[structopt(short = "V", long = "version")]
    pub version: bool,

    /// Print how long each step takes, among other things.
    /// Always enabled in debug builds.
    #[structopt(short = "v", long = "verbose")]
    pub debug: bool,

    /// Blur strength. Example: 10
    #[structopt(
        short = "b",
        long = "blur",
        raw(validator = "validators::greater_than(0)")
    )]
    pub radius: Option<u8>,

    /// Only place one icon. Default is to place an icon on each monitor. [NYI]
    #[structopt(long = "one-icon")]
    pub one_icon: bool,

    /// Interpret the icon as a mask, inverting masked pixels
    /// on the screenshot. Try it to see an example.
    #[structopt(long = "invert")]
    pub invert: bool,

    /// Icon placement, "center" to center,
    /// "x, y" (from top-left), or "-x,-y" (from bottom-right).
    /// Has no effect without --icon.
    /// Example: "(945, -20)"
    #[structopt(
        short = "u",
        long = "position",
        allow_hyphen_values = true,
        value_name = "coords|center",
        default_value = "Center"
    )]
    pub pos: Position,

    /// Path to icon to overlay on screenshot.
    #[structopt(
        short = "i",
        long = "icon",
        value_name = "file.png",
        parse(from_os_str),
        //raw(validator_os = "validators::is_png")
    )]
    pub path: Option<PathBuf>,

    /// Arguments to pass to i3lock. '--' must be used. Example: "-- --nofork
    /// --ignore-empty-password"
    #[structopt(
        value_name = "i3lock",
        takes_value = true,
        multiple = true,
        parse(from_os_str)
    )]
    pub i3lock: Vec<OsString>,
}

#[derive(Debug)]
pub enum Position {
    Center,
    Coords(isize, isize),
}

impl FromStr for Position {
    type Err = ParseIntError;

    // if you can improve this submit a PR
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("center") {
            return Ok(Position::Center);
        }
        let mut coords = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(str::trim);

        // TODO replace expect with IntErrorKind::Empty once it's stable
        let x = coords
            .next()
            .expect("--position takes exactly two integers")
            .parse::<isize>()?;
        let y = coords
            .next()
            .expect("--position takes exactly two integers")
            .parse::<isize>()?;
        assert!(
            coords.next().is_none(),
            "--position takes exactly two integers"
        );
        Ok(Position::Coords(x, y))
    }
}
