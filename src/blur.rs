use std::convert::TryFrom;
use std::num::{NonZeroU8, TryFromIntError};

use libc::c_int;

use imgref::ImgRefMut;

use rgb::alt::BGRA8;
use rgb::ComponentSlice;

extern "C" {
    fn stackblur(buffer: *mut u8, x: c_int, y: c_int, w: c_int, h: c_int, r: c_int, n: c_int);
}

pub trait Blur {
    unsafe fn blur(&mut self, radius: NonZeroU8) -> Result<(), TryFromIntError>;
}

impl Blur for ImgRefMut<'_, BGRA8> {
    #[must_use]
    unsafe fn blur(&mut self, radius: NonZeroU8) -> Result<(), TryFromIntError> {
        let w = c_int::try_from(self.width())?;
        let h = c_int::try_from(self.height())?;
        let r = c_int::from(radius.get());
        let n = c_int::try_from(num_cpus::get())?;

        stackblur(self.buf_mut().as_mut_slice().as_mut_ptr(), 0, 0, w, h, r, n);
        Ok(())
    }
}
