use std::error::Error;
use std::io::Error as IoError;
use std::{fmt, ptr, slice};

use xcb::randr;
use xcb::shm::attach_checked as xcb_attach;
use xcb::shm::detach_checked as xcb_detach;
use xcb::shm::get_image;
use xcb::{ConnError, Connection, Drawable, GenericError};

use self::I3LockrError::*;

macro_rules! handle_reply {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => return Err(XcbGeneric(e)),
        }
    };

    ($e:expr, $cleanup:stmt) => {
        match $e {
            Ok(r) => r,
            Err(e) => {
                $cleanup;
                return Err(XcbGeneric(e));
            }
        }
    };
}

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Screenshot<'a> {
    pub data: &'a mut [u8],
    pub monitors: Vec<(u16, u16)>, // can't access this via method or else borrow checker gets indigestion
    width: u16,
    height: u16,
    shmid: libc::c_int,
}

impl<'a> Screenshot<'a> {
    pub fn capture() -> Result<Self, I3LockrError> {
        let (conn, screen_num) = match Connection::connect(None) {
            Ok((c, n)) => (c, n),
            Err(e) => return Err(XcbConn(e)),
        };

        // connect to X11 and get some useful vars
        let setup = conn.get_setup();
        let screen = match setup.roots().nth(screen_num as usize) {
            Some(n) => n,
            None => unreachable!(),
        };

        // get handle on monitors
        let cookie = randr::get_screen_resources(&conn, screen.root());
        let reply = handle_reply!(cookie.get_reply());

        // grab their widths and heights
        // this silently throws away errors...
        #[allow(clippy::filter_map)]
        let monitors = reply
            .crtcs()
            .iter()
            .filter_map(|x| {
                randr::get_crtc_info(&conn, *x, reply.timestamp())
                    .get_reply()
                    .ok()
            })
            .filter(|x| x.mode() != 0)
            .map(|x| (x.width(), x.height()))
            .collect();

        // real work done here
        let (img, shmid) = match Self::real_capture(
            &conn,
            screen.root(),
            screen.width_in_pixels(),
            screen.height_in_pixels(),
        ) {
            Ok((sl, id)) => (sl, id),
            Err(e) => return Err(e),
        };

        Ok(Screenshot {
            data: img,
            monitors,
            width: screen.width_in_pixels(),
            height: screen.height_in_pixels(),
            shmid,
        })
    }

    fn real_capture(
        c: &Connection,
        d: Drawable,
        w: u16,
        h: u16,
    ) -> Result<(&'a mut [u8], libc::c_int), I3LockrError> {
        // setup POSIX SHM
        let shmid;
        unsafe {
            shmid = libc::shmget(
                libc::IPC_PRIVATE,
                w as usize * h as usize * 4,
                libc::IPC_CREAT | 0o600,
            );
        }
        if shmid == -1 {
            return Err(ShmIo(IoError::last_os_error()));
        }

        // generate XCB Segment
        let xid = c.generate_id();

        // attach X to SHM
        let buffer;
        let cookie = xcb_attach(c, xid, shmid as u32, false);
        handle_reply!(cookie.request_check(), unsafe {
            libc::shmctl(shmid, libc::IPC_RMID, ptr::null_mut());
        });

        // take screenshot
        let cookie = get_image(
            c, d, 0, 0, w, h, !0,   /* XAllPlanes */
            0x02, /* Z_PIXMAP */
            xid, 0,
        );
        handle_reply!(cookie.get_reply(), unsafe {
            libc::shmctl(shmid, libc::IPC_RMID, ptr::null_mut());
            xcb_detach(c, xid);
        });

        // detach
        let cookie = xcb_detach(c, xid);
        handle_reply!(cookie.request_check(), unsafe {
            libc::shmctl(shmid, libc::IPC_RMID, ptr::null_mut());
        });

        // we're done
        unsafe {
            buffer = libc::shmat(shmid, ptr::null(), 0);
            Ok((
                slice::from_raw_parts_mut(buffer as *mut u8, w as usize * h as usize * 4),
                shmid,
            ))
        }
    }

    pub const fn width(&self) -> u16 {
        self.width
    }

    pub const fn height(&self) -> u16 {
        self.height
    }
}

impl<'a> Drop for Screenshot<'a> {
    fn drop(&mut self) {
        unsafe {
            libc::shmdt(self.data.as_mut_ptr() as *mut libc::c_void);
            libc::shmctl(self.shmid, libc::IPC_RMID, ptr::null_mut());
        }
    }
}

#[derive(Debug)]
pub enum I3LockrError {
    XcbGeneric(GenericError),
    XcbConn(ConnError),
    ShmIo(IoError),
}

impl Error for I3LockrError {}

impl fmt::Display for I3LockrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XcbConn(e) => write!(f, "XCB {}", e),
            XcbGeneric(e) => write!(f, "XCB {}", e),
            ShmIo(e) => write!(f, "SHM I/O error: {}", e),
        }
    }
}
