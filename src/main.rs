use std::error::Error;
use std::io::ErrorKind::WouldBlock;
use std::io::{self, Write};
use std::num::TryFromIntError;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

use imgref::ImgRefMut;

use rgb::{ComponentBytes, FromSlice};

use scrap::{Capturer, Display, Frame};

use clap::Parser;

#[cfg(any(feature = "png", feature = "jpeg"))]
use xcb::Connection;

mod cli;
mod macros;

use cli::Cli;

#[cfg(any(feature = "png", feature = "jpeg"))]
use imagefmt::ColFmt;
#[cfg(any(feature = "png", feature = "jpeg"))]
use xcb::randr;
#[cfg(any(feature = "png", feature = "jpeg"))]
use xcb::Xid;

#[cfg(feature = "scale")]
mod scale;
#[cfg(feature = "scale")]
use scale::Scale;

#[cfg(feature = "blur")]
mod blur;
#[cfg(feature = "blur")]
use blur::Blur;

#[cfg(feature = "brightness")]
mod brightness;
#[cfg(feature = "brightness")]
use brightness::BrightnessAdj;

#[cfg(any(feature = "png", feature = "jpeg"))]
mod overlay;
#[cfg(any(feature = "png", feature = "jpeg"))]
use overlay::Compose;

fn main() -> Result<(), Box<dyn Error>> {
    timer_start!(everything);
    // parse args, handle custom `--version`
    let args = Cli::parse();
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

    #[cfg(any(feature = "png", feature = "jpeg"))]
    let (conn, screen_num) = Connection::connect(None)?;

    // setup scrap
    timer_start!(scrap);
    let disp = Display::primary()?;
    let mut capture = Capturer::new(disp)?;
    let (w, h) = (capture.width(), capture.height());
    timer_time!("Setting up scrap", scrap);

    // take the screenshot
    timer_start!(screenshot);
    let mut buffer: Frame;
    loop {
        match capture.frame() {
            Ok(buf) => {
                buffer = buf;
                break;
            }
            Err(e) => {
                if e.kind() == WouldBlock {
                    sleep(Duration::from_millis(33));
                    continue;
                }
            }
        }
    }
    timer_time!("Capturing screenshot", screenshot);

    // convert to imgref
    timer_start!(convert);
    let buf_bgra = buffer.as_bgra_mut();
    let mut screenshot = ImgRefMut::new(buf_bgra, w, h);
    timer_time!("Converting image", convert);

    // scaling is unsafe
    unsafe {
        time_routine!(
            screenshot,
            scale_down,
            args.factor,
            "scale",
            blur,
            args.radius,
            "blur",
            scale_up,
            args.factor,
            "scale",
            brighten,
            args.bright,
            "brightness",
            darken,
            args.dark,
            "brightness"
        );
    }

    // overlay/invert on each monitor
    if let Some(ref path) = args.path {
        #[cfg(any(feature = "png", feature = "jpeg"))]
        {
            timer_start!(decode);
            let image = imagefmt::read(path, ColFmt::BGRA)?;
            let image = imgref::ImgRef::new(image.buf.as_bgra(), image.w, image.h);
            timer_time!("Decoding overlay image", decode);

            // get handle on monitors
            let screen = conn
                .get_setup()
                .roots()
                .nth(usize::try_from(screen_num)?)
                .unwrap_or_else(|| unreachable!());

            let cookie = conn.send_request(&randr::GetScreenResources {
                window: screen.root(),
            });
            let reply = conn.wait_for_reply(cookie)?;

            let monitors = reply
                .crtcs()
                .iter()
                .filter_map(|crtc| {
                    let cookie = conn.send_request(&randr::GetCrtcInfo {
                        crtc: *crtc,
                        config_timestamp: reply.timestamp(),
                    });
                    conn.wait_for_reply(cookie).ok()
                })
                .enumerate()
                .filter(|(i, m)| !m.mode().is_none() && !args.ignore.contains(i))
                .map(|(_, m)| -> Result<_, _> {
                    Ok((
                        Ok::<_, TryFromIntError>(usize::from(m.width()))?,
                        Ok::<_, TryFromIntError>(usize::from(m.height()))?,
                        usize::try_from(m.x())?,
                        usize::try_from(m.y())?,
                    ))
                })
                .collect::<Result<Vec<(_, _, _, _)>, TryFromIntError>>()?;

            for (w, h, x, y) in monitors {
                let (x_off, y_off) = if args.pos.is_empty() {
                    if image.width() > w || image.height() > h {
                        eprintln!(
                            "Your image is larger than your monitor, image positions may be off!"
                        );
                    }
                    (
                        w / 2 - image.width() / 2 + x,
                        h / 2 - image.height() / 2 + y,
                    )
                } else {
                    unsafe {
                        (
                            wrap_to_screen(*args.pos.get_unchecked(0), w + x),
                            wrap_to_screen(*args.pos.get_unchecked(1), h + y),
                        )
                    }
                };

                debug!(
                    "Calculated image position on monitor: ({},{})",
                    x_off, y_off
                );

                timer_start!(overlay);
                if args.invert {
                    screenshot.invert(Some(image), x_off, y_off);
                } else {
                    screenshot.compose(image, x_off, y_off);
                }
                timer_time!("Overlaying image", overlay);
            }
        }
        #[cfg(not(any(feature = "png", feature = "jpeg")))]
        warn_disabled!("png/jpeg overlay");
    } else if args.invert {
        #[cfg(any(feature = "png", feature = "jpeg"))]
        {
            timer_start!(invert);
            screenshot.invert(None, 0, 0);
            timer_time!("Inverting image", invert);
        }
        #[cfg(not(any(feature = "png", feature = "jpeg")))]
        warn_disabled!("invert");
    }

    //TODO draw text

    // check if we're forking
    timer_start!(fork);
    let nofork = forking(args.i3lock.iter().map(String::as_str));
    timer_time!("Checking for nofork", fork);

    // call i3lock
    debug!("Calling i3lock with args: {:?}", args.i3lock);
    let mut cmd = Command::new("i3lock")
        .args([
            "-i",
            "/dev/stdin",
            //FIXME
            format!("--raw={w}x{h}:native").as_str(),
        ])
        .args(args.i3lock)
        .stdin(Stdio::piped())
        .spawn()?;

    // pass image bytes
    cmd.stdin
        .as_mut()
        .expect("Failed to take cmd.stdin.as_mut()")
        .write_all(screenshot.into_buf().as_bytes())?;

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
                .unwrap()
        )
        .into())
    }
}

// credit: @williewillus#8490
#[cfg(any(feature = "png", feature = "jpeg"))]
const fn wrap_to_screen(idx: isize, len: usize) -> usize {
    let pos = idx.unsigned_abs() % len;
    if pos == 0 {
        0
    } else {
        len - pos
    }
}

#[inline]
fn forking<'a, I>(mut args: I) -> bool
where
    I: Iterator<Item = &'a str>,
{
    args.any(|x| x == "--nofork" || (!x.starts_with("--") && x.contains('n')))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nofork() {
        assert!(forking(
            [
                "-n",
                "--insidecolor=542095ff",
                "--ringcolor=ffffffff",
                "--line-uses-inside"
            ]
            .into_iter()
        ));
        assert!(!forking(
            [
                "--insidecolor=542095ff",
                "--ringcolor=ffffffff",
                "--line-uses-inside"
            ]
            .into_iter()
        ));
        assert!(forking(
            [
                "--insidecolor=542095ff",
                "--ringcolor=ffffffff",
                "-en",
                "--line-uses-inside"
            ]
            .into_iter()
        ));
        assert!(!forking(
            [
                "--ringcolor=ffffffff",
                "-e",
                "--insidecolor=542095ff",
                "--line-uses-inside"
            ]
            .into_iter()
        ));
    }
}
