use blend_srgb::blend::blend_srgb8;

use imgref::ImgRef;
use imgref::ImgRefMut;

use rgb::alt::BGRA8;
use rgb::ColorComponentMap;

const MASK_THRESHOLD: u8 = 127;

pub trait Compose {
    fn compose(&mut self, top: ImgRef<BGRA8>, x: usize, y: usize);
    fn invert(&mut self, mask: Option<ImgRef<BGRA8>>, x: usize, y: usize);
}

impl Compose for ImgRefMut<'_, BGRA8> {
    fn compose(&mut self, top: ImgRef<BGRA8>, x: usize, y: usize) {
        let mut bot = self.sub_image_mut(x, y, top.width(), top.height());
        for (bot_px, top_px) in bot
            .pixels_mut()
            .zip(top.pixels())
            .filter(|(_, top_px)| top_px.a > 0)
        {
            if top_px.a == 255 {
                *bot_px = top_px;
            } else {
                let (b, g, r) = blend_srgb8(
                    (bot_px.b, bot_px.g, bot_px.r),
                    (top_px.b, top_px.g, top_px.r),
                    top_px.a,
                );
                *bot_px = BGRA8 { b, g, r, a: 255 };
            }
        }
    }

    fn invert(&mut self, mask: Option<ImgRef<BGRA8>>, x: usize, y: usize) {
        if let Some(m) = mask {
            let mut view = self.sub_image_mut(x, y, m.width(), m.height());
            for (view_px, _) in view
                .pixels_mut()
                .zip(m.pixels())
                .filter(|(_, mask_px)| mask_px.a > MASK_THRESHOLD)
            {
                *view_px = view_px.map_c(|c| !c);
            }
        } else {
            for pixel in self.pixels_mut() {
                *pixel = pixel.map_c(|c| !c);
            }
        }
    }
}
