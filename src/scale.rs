use imgref::ImgRefMut;

use itertools::iproduct;

use rgb::alt::BGRA8;

pub trait Scale {
    unsafe fn scale_up(&mut self, factor: usize);
    unsafe fn scale_down(&mut self, factor: usize);
}

impl Scale for ImgRefMut<'_, BGRA8> {
    unsafe fn scale_down(&mut self, factor: usize) {
        let (w, h) = (self.width(), self.height());
        for (y, x) in iproduct!(0..h / factor, 0..w / factor) {
            // we use this instead of index() to avoid bounds checks
            *self.buf_mut().get_unchecked_mut(y * w + x) =
                *self.buf().get_unchecked(y * factor * w + x * factor);
        }
    }

    unsafe fn scale_up(&mut self, factor: usize) {
        let (w, h) = (self.width(), self.height());
        for (y, x) in iproduct!((0..h).rev(), (0..w).rev()) {
            // we use this instead of index() to avoid bounds checks
            *self.buf_mut().get_unchecked_mut(y * w + x) =
                *self.buf().get_unchecked(y / factor * w + x / factor);
        }
    }
}
