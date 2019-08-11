use std::error::Error;
use std::ffi::OsStr;
use std::hint::unreachable_unchecked;
use std::io::{self, Write};
use std::panic;
use std::process::{Command, ExitStatus, Stdio};
use std::time::Instant;

use std::os::unix::process::ExitStatusExt;

use structopt::clap::Format;
use structopt::StructOpt;

use xcb::Connection;

mod blur;
mod cli;
mod macros;
mod pixels;

use cli::Cli;
use pixels::Pixels;

#[cfg(feature = "blur")]
mod ffi;

#[cfg(any(feature = "scale", feature = "png", feature = "jpeg"))]
mod algorithms;

#[cfg(any(feature = "png", feature = "jpeg"))]
use imagefmt::ColFmt;
#[cfg(any(feature = "png", feature = "jpeg"))]
use xcb::randr;

#[cfg(feature = "scale")]
use algorithms::Scale;

fn main() -> Result<(), Box<dyn Error>> {
    timer_start!(everything);
    // parse args, handle custom `--version`
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
        return Ok(());
    }

    // init debug macro
    macro_rules! debug {
        ($($arg:tt)*) => {
            if cfg!(debug_assertions) || args.verbose {
                eprintln!("{f}:{l}:{c} {fmt}", f=file!(), l=line!(), c=column!(), fmt=format!($($arg)*));
            }
        }
    }

    debug!("Found args: {:#?}", args);

    let (conn, screen_num) = Connection::connect(None)?;

    // take the screenshot
    timer_start!(screenshot);
    let mut shot = Pixels::capture(&conn, screen_num)?;
    timer_time!("Capturing screenshot", screenshot);

    debug!("Image is at /dev/shm{}", shot.path());

    if let Some(f) = args.factor {
        #[cfg(feature = "scale")]
        {
            timer_start!(downscale);
            shot.scale_down(f);
            timer_time!("Downscaling", downscale);
        }
        #[cfg(not(feature = "scale"))]
        warn_disabled!("scale");
    }

    if let Some(r) = args.radius {
        #[cfg(feature = "blur")]
        {
            timer_start!(blur);
            let (w, h) = shot.dimensions();
            ffi::blur(
                shot.as_bgra_8888_mut(),
                w as libc::c_int,
                h as libc::c_int,
                r as libc::c_int,
            );
            timer_time!("Blurring", blur);
        }
        #[cfg(not(feature = "blur"))]
        warn_disabled!("blur");
    }

    if let Some(f) = args.factor {
        #[cfg(feature = "scale")]
        {
            timer_start!(upscale);
            shot.scale_up(f);
            timer_time!("Upscaling", upscale);
        }
        #[cfg(not(feature = "scale"))]
        warn_disabled!("scale");
    }

    // overlay/invert on each monitor
    if let Some(ref path) = args.path {
        #[cfg(any(feature = "png", feature = "jpeg"))]
        {
            timer_start!(decode);
            let image = imagefmt::read(path, ColFmt::BGRA)?;
            timer_time!("Decoding overlay image", decode);

            // get handle on monitors
            let screen = conn
                .get_setup()
                .roots()
                .nth(screen_num as usize)
                .unwrap_or_else(|| unreachable!());

            let cookie = randr::get_screen_resources(&conn, screen.root());
            let reply = cookie.get_reply()?;

            for (w, h) in reply
                .crtcs()
                .iter()
                .filter_map(|x| {
                    randr::get_crtc_info(&conn, *x, reply.timestamp())
                        .get_reply()
                        .ok()
                })
                .enumerate()
                .filter(|(i, x)| x.mode() != 0 && !args.ignore.contains(&i))
                .map(|(_, x)| (usize::from(x.width()), usize::from(x.height())))
            {
                let (x_off, y_off) = if args.pos.is_empty() {
                    if image.w > w || image.h > h {
                        eprintln!(
                                "{}",
                                Format::Warning(
                                    "Your image is larger than your monitor, image positions may be off!"
                                    )
                                );
                    }
                    (w / 2 - image.w / 2, h / 2 - image.h / 2)
                } else {
                    unsafe {
                        (
                            wrap_to_screen(*args.pos.get_unchecked(0), w),
                            wrap_to_screen(*args.pos.get_unchecked(1), h),
                        )
                    }
                };

                debug!(
                    "Calculated image position on monitor: ({},{})",
                    x_off, y_off
                );

                timer_start!(overlay);
                algorithms::overlay(&mut shot, &image, x_off, y_off, args.invert);
                timer_time!("Overlaying image", overlay);
            }
        }
        #[cfg(not(any(feature = "png", feature = "jpeg")))]
        warn_disabled!("png/jpeg overlay");
    }

    //TODO draw text
    timer_start!(blur);
    blur::box_blur(shot.as_argb_32_mut(), 1920, 1080, 10);
    timer_time!("Blur", blur);

    // call i3lock and pass image bytes
    // this is a bit gross
    let nofork = args.i3lock.contains(&OsStr::new("-n").to_os_string())
        || args.i3lock.contains(&OsStr::new("--nofork").to_os_string());

    debug!("Calling i3lock with args: {:?}", args.i3lock);
    let mut cmd = Command::new("i3lock")
        .args(&[
            "-i",
            "/dev/stdin",
            &format!("--raw={}x{}:native", shot.width, shot.height),
        ])
        .args(args.i3lock)
        .stdin(Stdio::piped())
        .spawn()?;

    cmd.stdin
        .as_mut()
        .expect("Failed to take cmd.stdin.as_mut()")
        .write_all(shot.as_bgra_8888())?;

    timer_time!("Everything", everything);

    if nofork {
        debug!("Asked i3lock not to fork, calling wait()");
        match cmd.wait() {
            Ok(status) => status_to_result(status),
            Err(e) => Err(e.into()),
        }
    } else {
        match cmd.try_wait() {
            Ok(None) => Ok(()),
            Ok(Some(status)) => status_to_result(status),
            Err(e) => Err(e.into()),
        }
    }
}

fn status_to_result(status: ExitStatus) -> Result<(), Box<dyn Error>> {
    if status.success() {
        Ok(())
    } else if let Some(code) = status.code() {
        Err(io::Error::from_raw_os_error(code).into())
    } else {
        Err(format!(
            "Killed by signal: {}",
            status
                .signal()
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
        )
        .into())
    }
}

// credit: @williewillus#8490
#[cfg(any(feature = "png", feature = "jpeg"))]
fn wrap_to_screen(idx: isize, len: usize) -> usize {
    if idx.is_negative() {
        let pos = -idx as usize % len;
        if pos == 0 {
            0
        } else {
            len - pos
        }
    } else {
        idx as usize % len
    }
}
