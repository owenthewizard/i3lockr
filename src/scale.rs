use std::num::NonZeroUsize;

use imgref::ImgExt;
use imgref::ImgRefMut;

use itertools::iproduct;

pub trait Scale {
    unsafe fn scale_up(&mut self, factor: NonZeroUsize);
    unsafe fn scale_down(&mut self, factor: NonZeroUsize);
}

impl<T: Copy + Default> Scale for ImgRefMut<'_, T> {
    unsafe fn scale_down(&mut self, factor: NonZeroUsize) {
        let factor = factor.get();
        let (w, h) = (self.width(), self.height());
        let buf = self.buf_mut();
        for (y, x) in iproduct!(0..h / factor, 0..w / factor) {
            // we don't care what is left behind outside of our scaled image
            // take > swap > copy
            // in terms of speed
            let px = buf.get_unchecked_mut(y * factor * w + x * factor);
            let val = std::mem::take(px);
            let scaled_px = buf.get_unchecked_mut(y * w + x);
            *scaled_px = val;
        }
    }

    unsafe fn scale_up(&mut self, factor: NonZeroUsize) {
        let factor = factor.get();
        let (w, h) = (self.width_padded(), self.height_padded());
        let buf = self.buf_mut();
        for (y, x) in iproduct!((0..h).rev(), (0..w).rev()) {
            let i = y / factor * w + x / factor;
            buf.copy_within(i..=i, y * w + x);
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
