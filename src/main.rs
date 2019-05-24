use std::ffi::OsStr;
use std::io::Write;
use std::panic;
use std::process::{Command, Stdio};
use std::time::Instant;

use structopt::clap::Format;
use structopt::StructOpt;

mod cli;
use cli::Cli;
mod ffi;
mod screenshot;
use screenshot::Screenshot;
mod macros;
use macros::*;

fn main() {
    let args = Cli::from_args();
    if args.version {
        eprintln!(
            "{} v{} compiled for '{}' at {} ({}@{})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("TARGET"),
            env!("TIME"),
            env!("GIT_BRANCH"),
            env!("GIT_COMMIT")
        );
        return;
    }
    unsafe { DEBUG = args.debug };
    debug!("Found args: {:?}", args);

    timer_start!(screenshot);
    let shot =
        Screenshot::capture().unwrap_or_else(|e| color_panic!("Failed to take screenshot: {}", e));
    timer_time!("Capturing screenshot", screenshot);
    debug!("Found monitors: {:?}", shot.monitors());

    if args.radius.is_some() {
        timer_start!(blur);
        ffi::blur(
            shot.data,
            shot.width() as libc::c_int,
            shot.height() as libc::c_int,
            args.radius.unwrap_or_else(|| unreachable!()), // should be safe because validators ran already
        );
        timer_time!("Blurring", blur);
    }

    //TODO overlay + invert

    //TODO draw text

    // this is a bit gross
    let mut nofork = false;
    if args.i3lock.contains(&OsStr::new("-n").to_os_string())
        || args.i3lock.contains(&OsStr::new("--nofork").to_os_string())
    {
        nofork = true;
    }

    debug!("Calling i3lock with args: {:?}", args.i3lock);
    let mut cmd = Command::new("i3lock")
        .args(&[
            "-i",
            "/dev/stdin",
            &format!("--raw={}x{}:bgrx", shot.width(), shot.height()),
        ])
        .args(args.i3lock)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| color_panic!("Failed to call i3lock: {}", e));

    cmd.stdin
        .as_mut()
        .unwrap_or_else(|| color_panic!("Failed to open i3lock stdin!"))
        .write_all(shot.data)
        .unwrap_or_else(|e| color_panic!("Failed to write image to i3lock stdin: {}", e));

    if nofork {
        debug!("Asked i3lock not to fork, calling wait()");
        let _ = cmd.wait();
    }

    for pixel in shot.data.chunks_exact_mut(4) {
        unsafe { pixel.get_unchecked_mut(0..4).reverse() };
    }
}
