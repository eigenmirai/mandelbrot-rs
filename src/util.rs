use image::Rgb;

use crate::complex::Complex;

pub fn pixel_to_point(x: u32, y: u32, re0: f32, im0: f32, incr: f32) -> Complex {
    let re = (x as f32 * incr) + re0;
    let im = (y as f32 * incr) + im0;
    Complex::new(re, im)
}

pub fn average_colors(colors: Vec<Rgb<u8>>) -> [u8;3] {
    let mut ret: [u16;3] = [0;3];
    for c in colors.clone() {
        ret[0] += c.0[0] as u16;
        ret[1] += c.0[1] as u16;
        ret[2] += c.0[2] as u16;
    }
    ret.map(|e| (e / colors.len() as u16) as u8)
}

pub fn px_to_incr(scale: f32, width: u32) -> f32 {
    (1.0 / width as f32) * scale
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [u8; 3] {
    let c = v * s;
    let h_dash = h / 60.0;
    let x = c * (1.0 - (h_dash % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_dash < 1.0 {
        (c, x, 0.0)
    } else if h_dash < 2.0 {
        (x, c, 0.0)
    } else if h_dash < 3.0 {
        (0.0, c, x)
    } else if h_dash < 4.0 {
        (0.0, x, c)
    } else if h_dash < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}