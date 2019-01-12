use std::cmp::min;
use std::process::Command;
use std::time::Instant;

use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, value_t, App};

use image::DynamicImage::ImageRgba8;
use image::{imageops, DynamicImage, FilterType, GenericImageView, Pixel, Rgba, RgbaImage};

use mktemp::TempFile;

use xcb::{randr, Connection};

use xcb_util::image as xcb_img;

const LOCK_SCALE: u32 = 4;
const DEFAULT_LOCK: &[u8] = include_bytes!("../default.png");

// not sure if i should macro this or not
macro_rules! time_it {
    ($e:expr, $s:stmt) => {
        let now = Instant::now();
        $s;
        debug!("{} took {:#?}", $e, now.elapsed());
    };
}

macro_rules! debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!("{f}:{l}:{c} {fmt}", f=file!(), l=line!(), c=column!(), fmt=format!($($arg)*));
        }
    }
}

fn main() {
    // parse args, handle -h|-V
    // TODO validate args
    // validating args from yaml is impossible (afaik)
    // temporary hack: call value_t! early
    let yml = load_yaml!("cli.yaml");
    let vers = format!(
        "v{} ({}@{})",
        crate_version!(),
        env!("GIT_BRANCH"),
        env!("GIT_HASH")
    );

    let args = App::from_yaml(yml)
        .name(crate_name!())
        .version(&vers[..])
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    // parse necesary arguments
    let scale = value_t!(args, "scale", u32).unwrap_or_else(|e| panic!(e.message));
    let dark = value_t!(args, "dark", i32).unwrap_or_else(|e| panic!(e.message));
    let strength = value_t!(args, "strength", f32).unwrap_or_else(|e| panic!(e.message));
    let iter = value_t!(args, "iter", u8).unwrap_or_else(|e| panic!(e.message));

    // sanity check: we have a lock icon
    let lock = match args.occurrences_of("lock") {
        0 => match image::load_from_memory_with_format(DEFAULT_LOCK, image::ImageFormat::PNG) {
            Ok(img) => img,
            Err(e) => panic!("Error decoding default lock image: {}", e),
        },
        _ => match image::open(args.value_of("lock").unwrap()) {
            Ok(img) => img,
            Err(e) => panic!("Error opening lock image: {}", e),
        },
    };

    // take the screenshot
    let mut shot = ImageRgba8(take_screenshot());

    // process it
    process_screenshot(&mut shot, scale, dark, iter, strength);

    // scale lock image/mask to an appropriate size
    let size = min(shot.width(), shot.height()) / LOCK_SCALE;
    time_it!("Resizing lock", let lock = lock.resize(size, size, FilterType::Triangle).to_rgba());

    let mut shot = shot.to_rgba();
    draw_stuff(&mut shot, &lock, args.occurrences_of("invert") > 0);

    let outfile = match TempFile::new("i3lockr-", ".png") {
        Ok(tf) => tf,
        Err(e) => panic!("Failed to create temporary file: {}", e),
    };
    time_it!(
        "Exporting image",
        match shot.save(&outfile.path()) {
            Ok(()) => (),
            Err(e) => {
                panic!("Failed to export image: {}", e);
            }
        }
    );

    let mut args = match args.values_of("i3lock") {
        Some(args) => args.collect::<Vec<_>>(),
        None => Vec::with_capacity(2),
    };
    args.push("-i");
    args.push(&outfile.path());
    debug!("Calling i3lock with arguments: {:?}", args);
    let out = Command::new("i3lock").args(args).output().unwrap();

    debug!("{:?}", out);
}

fn process_screenshot(img: &mut DynamicImage, scale: u32, darkness: i32, blur_i: u8, blur_s: f32) {
    // scale it down
    if scale > 1 {
        time_it!(
            "Downscaling",
            *img = img.resize_exact(
                img.width() / scale,
                img.height() / scale,
                FilterType::Nearest
            )
        );
    }

    // darken it
    if darkness != 0 {
        time_it!("Darkening", *img = img.brighten(darkness));
    }

    // blur it
    if blur_s > 0.0 && blur_i > 0 {
        for i in 0..blur_i {
            time_it!(format!("Blurring pass {}", i + 1), *img = img.blur(blur_s));
        }
    }

    // scale it back up
    if scale > 1 {
        time_it!(
            "Upscaling",
            *img = img.resize_exact(
                img.width() * scale,
                img.height() * scale,
                FilterType::Triangle
            )
        );
    }
}

fn take_screenshot() -> RgbaImage {
    let (conn, screen_num) = match Connection::connect(None) {
        Ok((c, n)) => (c, n),
        Err(e) => panic!("Failed to open X11 display: {}", e),
    };
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();

    let now = Instant::now();
    let ximg = match xcb_img::get(
        &conn,
        root,
        0,
        0,
        screen.width_in_pixels(),
        screen.height_in_pixels(),
        !0u32,
        0x02u32,
    ) {
        Ok(img) => img,
        Err(()) => panic!("Failed to take screenshot!"),
    };
    debug!("Taking the screenshot took {:#?}", now.elapsed());

    let now = Instant::now();
    // @liftoff (Dan McDougall)
    let ret = RgbaImage::from_fn(
        screen.width_in_pixels() as u32,
        screen.height_in_pixels() as u32,
        |x, y| {
            let bgrx = ximg.get(x, y);
            let b = bgrx & 0x000000FF;
            let g = (bgrx & 0x0000FF00) >> 8;
            let r = (bgrx & 0x00FF0000) >> 16;
            Rgba::from_channels(r as u8, g as u8, b as u8, 255)
        },
    );
    debug!("Decoding the XImage took {:#?}", now.elapsed());
    ret
}

fn draw_stuff(img: &mut RgbaImage, icon: &RgbaImage, invert: bool) {
    let (conn, screen_num) = match Connection::connect(None) {
        Ok((c, n)) => (c, n),
        Err(e) => panic!("Failed to open X11 display: {}", e),
    };
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();

    let cookie = randr::get_screen_resources(&conn, root);
    let reply = match cookie.get_reply() {
        Ok(r) => r,
        Err(e) => panic!("Failed to query RandR screen resources: {}", e),
    };
    let crtcs = reply.crtcs();
    let mut crtc_cookies = Vec::with_capacity(crtcs.len());
    let timestamp = reply.timestamp();
    for crtc in crtcs {
        crtc_cookies.push(randr::get_crtc_info(&conn, *crtc, timestamp));
    }
    let now = Instant::now();
    for (i, crtc_cookie) in crtc_cookies.iter().enumerate() {
        let reply = match crtc_cookie.get_reply() {
            Ok(r) => r,
            Err(e) => panic!("Failed to query crtc info for CRTC-{}: {}", i, e),
        };

        // only get displays that are powered on/active
        if reply.mode() == 0 {
            continue;
        }

        let now = Instant::now();
        if invert {
            let mask = enlarge_image_canvas(icon, (*img).width(), (*img).height());
            for (mask_pixel, image_pixel) in
                mask.enumerate_pixels().zip((*img).enumerate_pixels_mut())
            {
                if mask_pixel.2[3] > 127 {
                    image_pixel.2.invert();
                }
            }
        } else {
            imageops::overlay(
                img,
                icon,
                reply.width() as u32 / 2 - (*icon).width() / 2,
                reply.height() as u32 / 2 - (*icon).height() / 2,
            );
        }
        /*
         * DRAW TEXT HERE
         */
        debug!("Drawing on CRTC-{} took {:#?}", i, now.elapsed());
    }
    debug!("Total drawing time: {:#?}", now.elapsed());
}

fn enlarge_image_canvas(icon: &RgbaImage, w: u32, h: u32) -> RgbaImage {
    let mut bot = RgbaImage::new(w, h);
    imageops::overlay(
        &mut bot,
        icon,
        w / 2 - (*icon).width() / 2,
        h / 2 - (*icon).height() / 2,
    );
    bot
}
