use std::num::NonZeroU8;

use imgref::ImgRefMut;

use rgb::alt::BGRA8;
use rgb::ComponentSlice;

use stackblur_iter::blur_srgb;

pub trait Blur {
    fn blur(&mut self, radius: NonZeroU8);
}

impl Blur for ImgRefMut<'_, BGRA8> {
    fn blur(&mut self, radius: NonZeroU8) {
        let (w, h) = (self.width(), self.height());
        let buf = unsafe { self.buf_mut().as_mut_slice().align_to_mut::<u32>().1 };
        let mut img = ImgRefMut::new(buf, w, h);
        blur_srgb(&mut img, radius.get().into());
    }
}
