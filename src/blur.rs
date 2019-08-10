use crate::pixels::Channels;
use crate::pixels::Pixels;

fn box_blur_vert(data: &mut [u8], width: usize, height: usize, radius: usize) {
    let iarr = 1.0 / ((radius + radius) as f32 + 1.0);

    for i in 0..width {
        let mut ti = i;
        let mut li = i;
        let mut ri = i + radius * width;

        let (fv, lv);
        unsafe {
            fv = isize::from(*data.get_unchecked(ti));
            lv = isize::from(*data.get_unchecked(ti + width * (height - 1)));
        }
        let mut val = ((radius + 1) as isize) * fv;

        for j in 0..radius {
            if let Some(v) = data.get(ti + j * width) {
                val += isize::from(*v);
            }
        }

        for j in 0..=radius {
            if let Some(v) = data.get(ri) {
                val += isize::from(*v) - fv;
            }

            if let Some(r) = data.get_mut(ti) {
                *r = round(val as f32 * iarr) as u8;
                ri += width;
                ti += width;
            }
        }

        for j in radius + 1..height - radius {
            if let (Some(v1), Some(v2)) = (data.get(ri), data.get(li)) {
                val += isize::from(*v1) - isize::from(*v2);
            }

            if let Some(r) = data.get_mut(ti) {
                *r = round(val as f32 * iarr) as u8;
                li += width;
                ri += width;
                ti += width;
            }
        }

        for j in height - radius..height {
            if let Some(v) = data.get(li) {
                val += lv - isize::from(*v);
            }

            if let Some(r) = data.get_mut(ti) {
                *r = round(val as f32 * iarr) as u8;
                li += width;
                ti += width;
            }
        }
    }
}

pub fn blur(image: &mut Pixels, width: usize, height: usize, radius: usize) {
    let mut buffer: Vec<u8>;

    buffer = image.channel_iter(Channels::Blue).copied().collect();
    box_blur_vert(buffer.as_mut_slice(), width, height, radius);
    //box_blur_horz
    for (dst, src) in image
        .channel_iter_mut(Channels::Blue)
        .zip(buffer.into_iter())
    {
        *dst = src;
    }

    buffer = image.channel_iter(Channels::Green).copied().collect();
    box_blur_vert(buffer.as_mut_slice(), width, height, radius);
    //box_blur_horz
    for (dst, src) in image
        .channel_iter_mut(Channels::Green)
        .zip(buffer.into_iter())
    {
        *dst = src;
    }

    buffer = image.channel_iter(Channels::Red).copied().collect();
    box_blur_vert(buffer.as_mut_slice(), width, height, radius);
    //box_blur_horz
    for (dst, src) in image
        .channel_iter_mut(Channels::Red)
        .zip(buffer.into_iter())
    {
        *dst = src;
    }
}

fn round(mut x: f32) -> f32 {
    x += 12582912.0;
    x -= 12582912.0;
    x
}
