use blend_srgb::blend::blend_srgb8;

use imgref::ImgRef;
use imgref::ImgRefMut;

use rgb::alt::BGRA8;
use rgb::ColorComponentMap;

pub trait Compose {
    fn compose(&mut self, top: ImgRef<BGRA8>, x: usize, y: usize);
    fn invert(&mut self, mask: Option<ImgRef<BGRA8>>, x: usize, y: usize);
}

impl Compose for ImgRefMut<'_, BGRA8> {
    fn compose(&mut self, top: ImgRef<BGRA8>, x: usize, y: usize) {
        let mut view = self.sub_image_mut(x, y, top.width(), top.height());
        // as below, doing this as rows so padding isn't included
        for (bot_row, top_row) in view.rows_mut().zip(top.rows()) {
            for (bot_pixel, top_pixel) in bot_row.iter_mut().zip(top_row.iter()) {
                let (b, g, r) = blend_srgb8(
                    (bot_pixel.b, bot_pixel.g, bot_pixel.r),
                    (top_pixel.b, top_pixel.g, top_pixel.r),
                    top_pixel.a,
                );
                *bot_pixel = BGRA8 { b, g, r, a: 255 };
            }
        }
    }

    fn invert(&mut self, mask: Option<ImgRef<BGRA8>>, x: usize, y: usize) {
        if let Some(m) = mask {
            let mut view = self.sub_image_mut(x, y, m.width(), m.height());
            // if we don't do this as rows padding is included
            for (view_row, mask_row) in view.rows_mut().zip(m.rows()) {
                for (view_pixel, mask_pixel) in view_row.iter_mut().zip(mask_row.iter()) {
                    if mask_pixel.a > 127 {
                        *view_pixel = view_pixel.map_c(|c| !c);
                    }
                }
            }
        } else {
            for pixel in self.buf_mut().iter_mut() {
                *pixel = pixel.map_c(|c| !c)
            }
        }
    }
}
