use std::num::NonZeroUsize;

use imgref::ImgExt;
use imgref::ImgRefMut;

use itertools::iproduct;

pub trait Scale {
    unsafe fn scale_up(&mut self, factor: NonZeroUsize);
    unsafe fn scale_down(&mut self, factor: NonZeroUsize);
}

impl<T: Copy> Scale for ImgRefMut<'_, T> {
    unsafe fn scale_down(&mut self, factor: NonZeroUsize) {
        let factor = factor.get();
        let (w, h) = (self.width(), self.height());
        for (y, x) in iproduct!(0..h / factor, 0..w / factor) {
            // we use this instead of index() to avoid bounds checks
            *self.buf_mut().get_unchecked_mut(y * w + x) =
                *self.buf().get_unchecked(y * factor * w + x * factor);
        }
    }

    unsafe fn scale_up(&mut self, factor: NonZeroUsize) {
        let factor = factor.get();
        let (w, h) = (self.width_padded(), self.height_padded());
        for (y, x) in iproduct!((0..h).rev(), (0..w).rev()) {
            // we use this instead of index() to avoid bounds checks
            *self.buf_mut().get_unchecked_mut(y * w + x) =
                *self.buf().get_unchecked(y / factor * w + x / factor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rgb::RGBA8;

    const RED: RGBA8 = RGBA8::new(255, 0, 0, 255);
    const GREEN: RGBA8 = RGBA8::new(0, 255, 0, 255);
    const BLUE: RGBA8 = RGBA8::new(0, 0, 255, 255);

    #[test]
    fn scale() {
        let mut data = vec![
            RED, RED, RED, RED, GREEN, GREEN, GREEN, GREEN, BLUE, BLUE, BLUE, BLUE, RED, RED, RED,
            RED,
        ];
        let mut img = ImgRefMut::new(data.as_mut(), 4, 4);
        unsafe { img.scale_down(NonZeroUsize::new(2).unwrap()) };
        dbg!(img.buf());
        assert_eq!(img.buf()[..2], [RED, RED]);
        assert_eq!(img.buf()[4..6], [BLUE, BLUE]);
        unsafe { img.scale_up(NonZeroUsize::new(2).unwrap()) };
        assert_eq!(
            img.buf(),
            &[
                RED, RED, RED, RED, RED, RED, RED, RED, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE,
                BLUE,
            ]
        );
    }
}
