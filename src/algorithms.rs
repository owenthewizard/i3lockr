#![cfg(any(
    feature = "scale",
    feature = "png",
    feature = "jpeg",
    feature = "brightness"
))]

use itertools::iproduct;

#[cfg(feature = "threads")]
use rayon::prelude::*;

use crate::pixels::Pixels;

#[cfg(any(feature = "png", feature = "jpeg"))]
const RB_MASK: u32 = 0x00_ff_00_ff;
#[cfg(any(feature = "png", feature = "jpeg"))]
const AG_MASK: u32 = !RB_MASK;
#[cfg(any(feature = "png", feature = "jpeg"))]
const AG_MASK_SHR: u32 = AG_MASK >> 8;
#[cfg(any(feature = "png", feature = "jpeg"))]
const ONE_ALPHA: u32 = 0x01 << 24;

#[cfg(feature = "scale")]
pub trait Scale {
    fn scale_up(&mut self, factor: usize);
    fn scale_down(&mut self, factor: usize);
}

#[cfg(feature = "scale")]
impl Scale for Pixels {
    // copied and modified from https://stackoverflow.com/a/28572644/5819375
    fn scale_down(&mut self, factor: usize) {
        let src_w = self.width;
        self.width /= factor;
        self.height /= factor;
        for (y, x) in iproduct!(0..self.height, 0..self.width) {
            let y2_xsource = y * factor * src_w;
            let i_xdest = y * self.width;
            let x2 = x * factor;

            unsafe {
                *self.as_argb_32_mut().get_unchecked_mut(i_xdest + x) =
                    *self.as_argb_32_mut().get_unchecked(y2_xsource + x2);
            }
        }
    }

    fn scale_up(&mut self, factor: usize) {
        let src_w = self.width;
        self.width *= factor;
        self.height *= factor;
        for (y, x) in iproduct!((0..self.height).rev(), (0..self.width).rev()) {
            let y2_xsource = y / factor * src_w;
            let i_xdest = y * self.width;
            let x2 = x / factor;

            unsafe {
                *self.as_argb_32_mut().get_unchecked_mut(i_xdest + x) =
                    *self.as_argb_32_mut().get_unchecked(y2_xsource + x2);
            }
        }
    }
}

#[cfg(any(feature = "png", feature = "jpeg"))]
pub fn overlay(
    bot: &mut Pixels,
    top: &imagefmt::Image<u8>,
    x_off: usize,
    y_off: usize,
    invert: bool,
) {
    let top_buf = unsafe { top.buf.as_slice().align_to::<u32>().1 };
    debug_assert!(top_buf.len() == top.buf.len() / 4);

    for (x, y) in iproduct!(0..top.w, 0..top.h) {
        let i_dst = x + x_off + bot.width * (y + y_off);
        let i_src = x + top.w * y;

        let src_argb = unsafe { *top_buf.get_unchecked(i_src) };
        let src_a = (src_argb >> 24) & 0xff;

        // skip invisible pixels
        if src_a == 0 {
            continue;
        }

        if let Some(dst_argb) = bot.as_argb_32_mut().get_mut(i_dst) {
            if invert {
                *dst_argb = !*dst_argb // doesn't this flip the alpha byte too?
            } else if src_a == 255 {
                *dst_argb = src_argb; // opaque pixels are a dumb copy
            } else {
                // anything else needs alpha blending
                // copied and modified from https://stackoverflow.com/a/27141669/5819375
                let na = 255 - src_a;
                let rb = ((na * (*dst_argb & RB_MASK)) + (src_a * (src_argb & RB_MASK))) >> 8;
                let ag = (na * ((*dst_argb >> 8) & AG_MASK_SHR))
                    + (src_a * (ONE_ALPHA | ((src_argb >> 8) & 0xff)));
                *dst_argb = (rb & RB_MASK) | (ag & AG_MASK);
            }
        }
    }
}

#[cfg(all(feature = "brightness", feature = "threads"))]
pub fn brighten(data: &mut [u8], factor: u8) {
    data.par_chunks_mut(4).for_each(|p| {
        let mut channels = p.iter_mut();
        let _ = channels.next_back(); // skip alpha
        channels.for_each(|x| *x = (*x).checked_add(factor).unwrap_or(255));
    })
}

#[cfg(all(feature = "brightness", not(feature = "threads")))]
pub fn brighten(data: &mut [u8], factor: u8) {
    data.chunks_exact_mut(4).for_each(|p| {
        let mut channels = p.iter_mut();
        let _ = channels.next_back(); // skip alpha
        channels.for_each(|x| *x = (*x).checked_add(factor).unwrap_or(255));
    })
}

#[cfg(all(feature = "brightness", feature = "threads"))]
pub fn darken(data: &mut [u8], factor: u8) {
    data.par_chunks_mut(4).for_each(|p| {
        let mut channels = p.iter_mut();
        let _ = channels.next_back(); // skip alpha
        channels.for_each(|x| *x = (*x).checked_sub(factor).unwrap_or(0));
    })
}

#[cfg(all(feature = "brightness", not(feature = "threads")))]
pub fn darken(data: &mut [u8], factor: u8) {
    data.chunks_exact_mut(4).for_each(|p| {
        let mut channels = p.iter_mut();
        let _ = channels.next_back(); // skip alpha
        channels.for_each(|x| *x = (*x).checked_sub(factor).unwrap_or(0));
    })
}
