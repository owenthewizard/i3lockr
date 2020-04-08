use std::convert::TryInto;
use std::hint::unreachable_unchecked;

use imgref::ImgRef;
use imgref::ImgRefMut;

use rgb::alt::BGRA8;
use rgb::ComponentSlice;

const RB_MASK: u32 = 0x00_ff_00_ff;
const AG_MASK: u32 = !RB_MASK;
const AG_MASK_SHR: u32 = AG_MASK >> 8;
const ONE_ALPHA: u32 = 0x01 << 24;

pub trait Compose {
    fn compose(&mut self, top: ImgRef<BGRA8>, x: usize, y: usize);
    fn invert(&mut self, mask: Option<ImgRef<BGRA8>>, x: usize, y: usize);
}

impl Compose for ImgRefMut<'_, BGRA8> {
    fn compose(&mut self, top: ImgRef<BGRA8>, x: usize, y: usize) {
        let mut view = self.sub_image_mut(x, y, top.width(), top.height());
        // we may want this later if composing an image without alpha
        /*
        for (bot_row, top_row) in view.rows_mut().zip(top.rows()) {
            bot_row.copy_from_slice(top_row);
        */
        // as below, doing this as rows so padding isn't included
        for (bot_row, top_row) in view.rows_mut().zip(top.rows()) {
            for (bot_pixel, top_pixel) in bot_row.iter_mut().zip(top_row.iter()) {
                match top_pixel.a {
                    255 => continue,                                 // invisible
                    0 => *bot_pixel = *top_pixel,                    // opaque
                    _ => *bot_pixel = blend(*bot_pixel, *top_pixel), // alpha blend
                }
            }
        }
    }

    fn invert(&mut self, mask: Option<ImgRef<BGRA8>>, x: usize, y: usize) {
        if let Some(m) = mask {
            let mut view = self.sub_image_mut(x, y, m.width(), m.height());
            // if we don't do this as rows padding is included
            for (view_row, mask_row) in view.rows_mut().zip(m.rows()) {
                for (view_pixel, mask_pixel) in view_row.iter_mut().zip(mask_row.iter()) {
                    if mask_pixel.a < 127 {
                        for channel in view_pixel.bgr_mut().as_mut_slice().iter_mut() {
                            *channel = !*channel;
                        }
                    }
                }
            }
        } else {
            for pixel in self.buf_mut().iter_mut() {
                for channel in pixel.bgr_mut().as_mut_slice().iter_mut() {
                    *channel = !*channel;
                }
            }
        }
    }
}

fn blend(px_a: BGRA8, px_b: BGRA8) -> BGRA8 {
    let bgra_1 = u32::from_ne_bytes(
        px_a.as_slice()
            .try_into()
            .unwrap_or_else(|_| unsafe { unreachable_unchecked() }),
    );
    let bgra_2 = u32::from_ne_bytes(
        px_b.as_slice()
            .try_into()
            .unwrap_or_else(|_| unsafe { unreachable_unchecked() }),
    );

    let src_a = u32::from(px_b.a);
    let na = 255 - src_a;

    let rb = ((na * (bgra_1 & RB_MASK)) + (src_a * (bgra_2 & RB_MASK))) >> 8;
    let ag = (na * ((bgra_1 >> 8) & AG_MASK_SHR)) + (src_a * (ONE_ALPHA | ((bgra_2 >> 8) & 0xff)));

    BGRA8::from(((rb & RB_MASK) | (ag & AG_MASK)).to_ne_bytes())
}
