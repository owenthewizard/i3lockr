use std::num::NonZeroUsize;

use imgref::ImgRefMut;

use rgb::alt::BGRA8;
use rgb::ComponentSlice;

#[cfg(not(any(feature = "threads", feature = "simd")))]
use stackblur_iter::blur_srgb;
#[cfg(all(feature = "threads", not(feature = "simd")))]
use stackblur_iter::par_blur_srgb as blur_srgb;
#[cfg(all(feature = "simd", not(feature = "threads")))]
use stackblur_iter::simd_blur_srgb as blur_srgb;
#[cfg(all(feature = "simd", feature = "threads"))]
use stackblur_iter::par_simd_blur_srgb as blur_srgb;

pub trait Blur {
    fn blur(&mut self, radius: NonZeroUsize);
}

impl Blur for ImgRefMut<'_, BGRA8> {
    fn blur(&mut self, radius: NonZeroUsize) {
        let (w, h) = (self.width(), self.height());
        let buf = unsafe { self.buf_mut().as_mut_slice().align_to_mut::<u32>().1 };
        let mut img = ImgRefMut::new(buf, w, h);
        #[cfg(not(feature = "simd"))]
        blur_srgb(&mut img, radius.get());
        #[cfg(feature = "simd")]
        blur_srgb::<32>(&mut img, radius.get());
    }
}
