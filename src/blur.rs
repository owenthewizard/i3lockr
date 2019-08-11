use std::cmp::min;
use std::slice;

unsafe fn box_blur_vert(data: &mut [u32], width: usize, height: usize, blur_radius: usize) {
    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;

    for i in 0..width {
        let col_start = i; //inclusive
        let col_end = i + width * (height - 1); //inclusive
        let mut ti: usize = i;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius * width;

        let fv = get_rgb(data.get_unchecked(col_start));
        let lv = get_rgb(data.get_unchecked(col_end));

        let mut val_r: isize = (blur_radius as isize + 1) * isize::from(fv[0]);
        let mut val_g: isize = (blur_radius as isize + 1) * isize::from(fv[1]);
        let mut val_b: isize = (blur_radius as isize + 1) * isize::from(fv[2]);

        // Get the pixel at the specified index, or the first pixel of the column
        // if the index is beyond the top edge of the image
        let get_top = |i: usize| {
            if i < col_start {
                fv
            } else {
                get_rgb(data.get_unchecked(i))
            }
        };

        // Get the pixel at the specified index, or the last pixel of the column
        // if the index is beyond the bottom edge of the image
        let get_bottom = |i: usize| {
            if i > col_end {
                lv
            } else {
                get_rgb(data.get_unchecked(i))
            }
        };

        for j in 0..min(blur_radius, height) {
            let bb = get_rgb(data.get_unchecked(ti + j * width));
            val_r += isize::from(bb[0]);
            val_g += isize::from(bb[1]);
            val_b += isize::from(bb[2]);
        }
        if blur_radius > height {
            val_r += (blur_radius - height) as isize * isize::from(lv[0]);
            val_g += (blur_radius - height) as isize * isize::from(lv[1]);
            val_b += (blur_radius - height) as isize * isize::from(lv[2]);
        }

        for _ in 0..min(height, blur_radius + 1) {
            let bb = if ri > col_end {
                lv
            } else {
                get_rgb(data.get_unchecked(ri))
            };

            ri += width;
            val_r += isize::from(bb[0]) - isize::from(fv[0]);
            val_g += isize::from(bb[1]) - isize::from(fv[1]);
            val_b += isize::from(bb[2]) - isize::from(fv[2]);

            set_bgr(
                data.get_unchecked_mut(ti),
                &[
                    round(val_b as f32 * iarr) as u8,
                    round(val_g as f32 * iarr) as u8,
                    round(val_r as f32 * iarr) as u8,
                ],
            );
            ti += width;
        }

        if height > blur_radius {
            // otherwise `(height - blur_radius)` will underflow
            for _ in (blur_radius + 1)..(height - blur_radius) {
                let bb1 = get_rgb(data.get_unchecked(ri));
                ri += width;
                let bb2 = get_rgb(data.get_unchecked(li));
                li += width;

                val_r += isize::from(bb1[0]) - isize::from(bb2[0]);
                val_g += isize::from(bb1[1]) - isize::from(bb2[1]);
                val_b += isize::from(bb1[2]) - isize::from(bb2[2]);

                set_bgr(
                    data.get_unchecked_mut(ti),
                    &[
                        round(val_b as f32 * iarr) as u8,
                        round(val_g as f32 * iarr) as u8,
                        round(val_r as f32 * iarr) as u8,
                    ],
                );
                ti += width;
            }

            for _ in 0..min(height - blur_radius - 1, blur_radius) {
                //let bb = get_top(li);
                let bb = {
                    if li < col_start {
                        fv
                    } else {
                        get_rgb(data.get_unchecked(li))
                    }
                };
                li += width;

                val_r += isize::from(lv[0]) - isize::from(bb[0]);
                val_g += isize::from(lv[1]) - isize::from(bb[1]);
                val_b += isize::from(lv[2]) - isize::from(bb[2]);

                set_bgr(
                    data.get_unchecked_mut(ti),
                    &[
                        round(val_b as f32 * iarr) as u8,
                        round(val_g as f32 * iarr) as u8,
                        round(val_r as f32 * iarr) as u8,
                    ],
                );
                ti += width;
            }
        }
    }
}

pub fn box_blur(data: &mut [u32], width: usize, height: usize, radius: usize) {
    unsafe {
        box_blur_vert(data, width, height, radius);
    }
}

const fn get_rgb(pixel: &u32) -> [u8; 3] {
    [
        ((*pixel >> 16) & 0xff) as u8,
        ((*pixel >> 8) & 0xff) as u8,
        (*pixel & 0xff) as u8,
    ]
}

unsafe fn set_bgr(pixel: &mut u32, rgb: &[u8]) {
    //TODO check this skips alpha and not blue
    let sl = slice::from_raw_parts_mut((pixel as *mut _ as *mut u8), 3);
    sl.copy_from_slice(rgb);
}

fn round(mut x: f32) -> f32 {
    x += 12582912.0;
    x -= 12582912.0;
    x
}
