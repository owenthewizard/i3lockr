use std::num::NonZeroU8;

use imgref::ImgRefMut;

#[cfg(feature = "threads")]
use rayon::prelude::*;

use rgb::alt::BGRA8;
use rgb::{ColorComponentMap, ComponentSlice};

pub trait BrightnessAdj {
    fn brighten(&mut self, amt: NonZeroU8);
    fn darken(&mut self, amt: NonZeroU8);
}

impl BrightnessAdj for ImgRefMut<'_, BGRA8> {
    #[cfg(feature = "threads")]
    fn brighten(&mut self, amt: NonZeroU8) {
        // need to access buffer manually because imgref doesn't support rayon
        self.buf_mut().par_iter_mut().for_each(|pixel| {
            *pixel = pixel.map_c(|c| c.saturating_add(amt.get()));
        });
    }

    #[cfg(not(feature = "threads"))]
    fn brighten(&mut self, amt: NonZeroU8) {
        // need to access buffer manually because imgref doesn't support rayon
        self.buf_mut().iter_mut().for_each(|pixel| {
            *pixel = pixel.map_c(|c| c.saturating_add(amt.get()));
        });
    }

    #[cfg(feature = "threads")]
    fn darken(&mut self, amt: NonZeroU8) {
        // need to access buffer manually because imgref doesn't support rayon
        self.buf_mut().par_iter_mut().for_each(|pixel| {
            *pixel = pixel.map_c(|c| c.saturating_sub(amt.get()));
        });
    }

    #[cfg(not(feature = "threads"))]
    fn darken(&mut self, amt: NonZeroU8) {
        // need to access buffer manually because imgref doesn't support rayon
        self.buf_mut().iter_mut().for_each(|pixel| {
            *pixel = pixel.map_c(|c| c.saturating_sub(amt.get()));
        });
    }
}
