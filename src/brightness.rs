use std::num::NonZeroU8;

use imgref::ImgRefMut;

#[cfg(feature = "threads")]
use rayon::prelude::*;

use rgb::alt::BGRA8;
use rgb::ColorComponentMap;

pub trait BrightnessAdj {
    fn brighten(&mut self, amt: NonZeroU8);
    fn darken(&mut self, amt: NonZeroU8);
}

impl BrightnessAdj for ImgRefMut<'_, BGRA8> {
    fn brighten(&mut self, amt: NonZeroU8) {
        #[cfg(not(feature = "threads"))]
        for pixel in self.pixels_mut() {
            *pixel = pixel.map_c(|c| c.saturating_add(amt.get()));
        }

        #[cfg(feature = "threads")]
        self.rows_mut().par_bridge().for_each(|row| {
            for pixel in row.iter_mut() {
                *pixel = pixel.map_c(|c| c.saturating_add(amt.get()));
            }
        });
    }

    fn darken(&mut self, amt: NonZeroU8) {
        #[cfg(not(feature = "threads"))]
        for pixel in self.pixels_mut() {
            *pixel = pixel.map_c(|c| c.saturating_sub(amt.get()));
        }

        #[cfg(feature = "threads")]
        self.rows_mut().par_bridge().for_each(|row| {
            for pixel in row.iter_mut() {
                *pixel = pixel.map_c(|c| c.saturating_sub(amt.get()));
            }
        });
    }
}
