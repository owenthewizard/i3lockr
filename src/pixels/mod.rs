use std::ffi::CString;
use std::hint::unreachable_unchecked;
use std::io::Error as IoError;
use std::{ptr, slice};

use libc::{c_int, c_void, close, ftruncate, mmap, munmap, off_t, shm_open, shm_unlink};
use libc::{
    MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, S_IRUSR,
    S_IWUSR,
};

use xcb::shm::attach_fd_checked;
use xcb::shm::detach_checked;
use xcb::shm::get_image;
use xcb::Connection;

#[macro_use]
mod error;
use error::I3LockrError::{self, *};

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Pixels {
    pub width: usize,
    pub height: usize,
    size: usize,
    name: CString,
    fd: c_int,
    addr: *mut c_void,
}

impl Pixels {
    pub fn capture(conn: &Connection, screen_num: c_int) -> Result<Self, I3LockrError> {
        let screen = match conn.get_setup().roots().nth(screen_num as usize) {
            Some(n) => n,
            None => unreachable!(),
        };
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
                return Err(ShmOpen(IoError::last_os_error()));
            }

            let err = ftruncate(fd, size as off_t);
            if err != 0 {
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                return Err(FTruncate(IoError::last_os_error()));
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
                let _ = close(fd);
                let _ = shm_unlink(name.as_ptr());
                return Err(MMap(IoError::last_os_error()));
            }
        }

        // attach X to SHM
        let cookie = attach_fd_checked(conn, xid, fd, false);
        handle_reply!(cookie.request_check(), unsafe {
            let _ = munmap(addr, size);
            let _ = close(fd);
            let _ = shm_unlink(name.as_ptr());
        });

        // take screenshot
        let cookie = get_image(
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
        );

        handle_reply!(cookie.get_reply(), unsafe {
            let _ = munmap(addr, size);
            let _ = close(fd);
            let _ = shm_unlink(name.as_ptr());
            detach_checked(conn, xid);
        });

        // detach
        let cookie = detach_checked(conn, xid);
        handle_reply!(cookie.request_check(), unsafe {
            let _ = munmap(addr, size);
            let _ = close(fd);
            let _ = shm_unlink(name.as_ptr());
        });

        Ok(Self {
            width,
            height,
            size,
            name,
            fd,
            addr,
        })
    }

    pub fn as_argb_8888_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.addr as *mut u8, self.size) }
    }

    pub fn as_argb_32_mut(&mut self) -> &mut [u32] {
        debug_assert!(unsafe {
            let (head, _, tail) = self.as_argb_8888_mut().align_to_mut::<u32>();
            head.is_empty() && tail.is_empty()
        });
        unsafe { self.as_argb_8888_mut().align_to_mut::<u32>().1 }
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
