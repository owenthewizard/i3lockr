use std::ffi::OsStr;
use std::io::Write;
use std::panic;
use std::process::{Command, Stdio};
use std::time::Instant;

use imagefmt::ColFmt;

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

    if let Some(r) = args.radius {
        timer_start!(blur);
        ffi::blur(
            shot.data,
            shot.width() as libc::c_int,
            shot.height() as libc::c_int,
            r as libc::c_int,
        );
        timer_time!("Blurring", blur);
    }

    //TODO invert
    if let Some(path) = args.path {
        timer_start!(decode);
        let image = imagefmt::read(path, ColFmt::BGRA)
            .unwrap_or_else(|e| color_panic!("Failed to read image: {}", e));
        timer_time!("Decoding image", decode);
        let (mut x_off, mut y_off) = match args.pos {
            cli::Position::Center => (
                (shot.width() as usize / 2 - image.w / 2) as isize, // isize because Coords below are isize and match blocks must be homogeneous
                (shot.height() as usize / 2 - image.h / 2) as isize,
            ),
            cli::Position::Coords(x, y) => (x, y),
        };

        while x_off.is_negative() {
            x_off += shot.width() as isize;
        }
        while y_off.is_negative() {
            y_off += shot.height() as isize;
        }
        while x_off >= shot.width() as isize {
            x_off -= shot.width() as isize;
        }
        while y_off >= shot.height() as isize {
            y_off -= shot.height() as isize;
        }

        let (x_off, y_off) = (x_off as usize, y_off as usize);
        debug!("Calculated image position: ({},{})", x_off, y_off);

        // should be able to rewrite this to write rows at once
        timer_start!(overlay);
        for x in 0..image.w {
            for y in 0..image.h {
                let i_dst = (x + x_off + shot.width() as usize * (y + y_off)) * 4;
                let i_src = (x + image.w * y) * 4;
                let src = unsafe { image.buf.get_unchecked(i_src..i_src + 4) };
                let dst = shot.data.get_mut(i_dst..i_dst + 4);

                if let Some(sl) = dst {
                    if args.invert {
                        match unsafe { src.get_unchecked(3) } {
                            0 => continue,
                            _ => unsafe {
                                sl.get_unchecked_mut(0..4).iter_mut().for_each(|p| *p = !*p)
                            },
                        }
                    } else {
                        match unsafe { src.get_unchecked(3) } {
                            // alpha byte
                            0 => continue,                   // skip transparent pixels
                            255 => sl.copy_from_slice(&src), // opaque pixels are a dumb copy
                            _ => {
                                // anything else need alpha blending
                                unsafe {
                                    let a = *src.get_unchecked(3) as usize + 1;
                                    let inv_a = 256 - *src.get_unchecked(3) as usize;
                                    sl.get_unchecked_mut(0..4)
                                        .iter_mut()
                                        .zip(src.get_unchecked(0..4).iter())
                                        .for_each(|(dst_p, src_p)| {
                                            *dst_p = ((a * *dst_p as usize
                                                + inv_a * *src_p as usize)
                                                >> 8)
                                                as u8; // this overflows...
                                        });
                                }
                            }
                        }
                    }
                }
            }
        }
        timer_time!("Overlaying/inverting image", overlay);
    }

    //TODO draw text

    // this is a bit gross
    let nofork = args.i3lock.contains(&OsStr::new("-n").to_os_string())
        || args.i3lock.contains(&OsStr::new("--nofork").to_os_string());

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
}
