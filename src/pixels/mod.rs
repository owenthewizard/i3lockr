use std::ffi::CString;
use std::hint::unreachable_unchecked;
use std::io::Error as IoError;
use std::{ptr, slice};

use libc::{
    c_int, c_void, close, ftruncate, mmap, munmap, off_t, shm_open, shm_unlink, MAP_FAILED,
    MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, S_IRUSR, S_IWUSR,
};

use xcb::shm::attach_fd_checked;
use xcb::shm::detach_checked;
use xcb::shm::get_image;
use xcb::Connection;

mod error;
use error::CaptureError::{self, LibcFunc};

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Pixels {
    pub width: usize,
    pub height: usize,
    size: usize,
    name: CString,
    fd: c_int,
    addr: *mut c_void,
}

pub enum Channels {
    Blue = 0,
    Green = 1,
    Red = 2,
    Alpha = 3,
}

impl Pixels {
    pub fn capture(conn: &Connection, screen_num: c_int) -> Result<Self, CaptureError> {
        let screen = conn
            .get_setup()
            .roots()
            .nth(screen_num as usize)
            .unwrap_or_else(|| unreachable!());

        let (width, height) = (
            screen.width_in_pixels() as usize,
            screen.height_in_pixels() as usize,
        );
        let size = width * height * 4; // assuming 4 bytes per pixel (8 bits per channel, including alpha)

        // generate XCB Segment
        // capture it now so we can use it as """entropy""" for the file path
        let xid = conn.generate_id();

        // map an SHM file into memory
        let (name, fd, addr);
        let my_vec = format!("/i3lockr-{}.dat", xid).into_bytes();
        unsafe {
            name = CString::from_vec_unchecked(my_vec);
            fd = shm_open(name.as_ptr(), O_RDWR | O_CREAT | O_EXCL, S_IRUSR | S_IWUSR);
            if fd < 0 {
                return Err(LibcFunc("shm_open".to_owned(), IoError::last_os_error()));
            }

            let err = ftruncate(fd, size as off_t);
            if err != 0 {
                let err = IoError::last_os_error();
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                return Err(LibcFunc("ftruncate".to_owned(), err));
            }

            addr = mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED_VALIDATE,
                fd,
                0,
            );
            if addr == MAP_FAILED {
                let err = IoError::last_os_error();
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                return Err(LibcFunc("mmap".to_owned(), err));
            }
        }

        // attach X to SHM
        if let Err(e) = attach_fd_checked(conn, xid, fd, false).request_check() {
            unsafe {
                let _ = munmap(addr, size);
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                return Err(e.into());
            }
        }

        // take screenshot
        if let Err(e) = get_image(
            conn,
            screen.root(),
            0,
            0,
            width as u16,
            height as u16,
            !0,   /* XAllPlanes */
            0x02, /* Z_PIXMAP */
            xid,
            0,
        )
        .get_reply()
        {
            unsafe {
                let _ = munmap(addr, size);
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                xcb::shm::detach(conn, xid);
                return Err(e.into());
            }
        }

        // detach
        if let Err(e) = detach_checked(conn, xid).request_check() {
            unsafe {
                let _ = munmap(addr, size);
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                return Err(e.into());
            }
        }

        Ok(Self {
            width,
            height,
            size,
            name,
            fd,
            addr,
        })
    }

    pub fn as_argb_32_mut(&mut self) -> &mut [u32] {
        unsafe { slice::from_raw_parts_mut(self.addr as *mut u32, self.size) }
    }

    pub fn as_bgra_8888(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.addr as *const u8, self.size) }
    }

    pub fn as_bgra_8888_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr as *mut u8, self.size) }
    }

    pub const fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub const fn size(&self) -> usize {
        self.size
    }

    pub fn path(&self) -> &str {
        self.name
            .as_c_str()
            .to_str()
            .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
    }

    pub fn into_planar(&mut self) {
        let mut pixel_order = (0..4).cycle();
        self.as_bgra_8888_mut()
            .sort_by_cached_key(|_| pixel_order.next());
    }

    pub fn channel_iter(&self, channel: Channels) -> impl Iterator<Item = &u8> {
        self.as_bgra_8888().iter().skip(channel as usize).step_by(4)
    }

    pub fn channel_iter_mut(&mut self, channel: Channels) -> impl Iterator<Item = &mut u8> {
        self.as_bgra_8888_mut()
            .iter_mut()
            .skip(channel as usize)
            .step_by(4)
    }
}

impl Drop for Pixels {
    fn drop(&mut self) {
        unsafe {
            let _ = munmap(self.addr, self.size);
            let _ = close(self.fd);
            let _ = shm_unlink(self.name.as_ptr());
        }
    }
}
