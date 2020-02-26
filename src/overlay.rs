use imgref::ImgRef;
use imgref::ImgRefMut;

use rayon::prelude::*;

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
        for (bot_row, top_row) in view.rows_mut().zip(top.rows()) {
            bot_row.copy_from_slice(top_row);
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

fn blend(px_a: BGRA8, px_b: BGRA8) {}
