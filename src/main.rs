#[allow(dead_code, non_snake_case, non_upper_case_globals)]

mod complex;
mod util;

use util::*;
use complex::Complex;
use std::time::Instant;

use image::{RgbImage, Rgb};
use clap::Parser;
use rayon::prelude::*;

const LIMIT_SQ: f32 = 4.0;

fn main() {
    let args = Args::parse();
    let t0 = Instant::now();
    let img = render_mandelbrot(&args);
    let elapsed = t0.elapsed();

    img.save(args.output).unwrap();
    println!("{}ms", elapsed.as_millis())
}

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', default_value_t = 2560)]
    width: u32,

    #[arg(short = 'H', default_value_t = 1920)]
    height: u32,

    // lowest real value
    #[arg(short = 'x', default_value_t = -2.0)]
    re0: f32,
    // lowest imaginary value
    #[arg(short = 'y', default_value_t = -1.12)]
    im0: f32,

    // scale
    #[arg(short = 's', default_value_t = 3.0)]
    scale: f32,

    #[arg(short = 'i', default_value_t = 256)]
    iterations: u32,

    #[arg(short = 'o', default_value_t = String::from("output.png"))]
    output: String,

    // antialiasing
    #[arg(short = 'A', default_value_t = 1)]
    AA: u32,
}

fn render_mandelbrot(args: &Args) -> RgbImage {
    let mut img = RgbImage::new(args.width, args.height);
    let incr = px_to_incr(args.scale, args.width);
    let AA = args.AA;

    img.enumerate_pixels_mut().par_bridge().for_each(|(x, y, p)| {
        let x = x * AA;
        let y = y * AA;
        let incr = incr / AA as f32;

        let mut colors: Vec<Rgb<u8>> = vec![];
        for i in 0..AA {
            for j in 0..AA {
                let c = pixel_to_point(x + i, y + j, args.re0, args.im0, incr);
                let m = mandelbrot(c, args.iterations);

                colors.push(color(m.0, m.1, args.iterations))
            }
        }
        *p = Rgb(average_colors(colors));
    });
    img
}

// this function returns the amount of iterations before z_n.abs() is above 2
fn mandelbrot(c: Complex, iterations: u32) -> (u32, Complex) {
    let mut z = Complex::new(0f32, 0f32);
    let mut n: u32 = 0;
    
    // store old value to detect cycles, update every 16 iterations
    let mut z_old = z;

    while z.abs_sq() <= LIMIT_SQ && n < iterations {
        if n % 16 == 0 {
            z_old = z;
        }
        z = z.sq() + c;
        if z.fuzzy_eq(z_old) {
            n = iterations;
            break;
        }
        n += 1;
    }

    // 4 more iterations
    for _e in 0..4 {
        z = z.sq() + c;
    }    
    return (n, z);
}

const log_base: f32 = 1.0 / 0.69314718056;
const K: f32 = 6.0;

fn color(n: u32, z: Complex, max_iter: u32) -> Rgb<u8> {
    if n == max_iter {
        return Rgb([0, 0, 0]);
    }
    let if32 = max_iter as f32;
    let v = K + n as f32 - z.abs_sq().ln().ln()*log_base;
    let mut c = hsv_to_rgb(360.0*v/if32, 1.0, 10.0*v/if32);
    let t = c[0];
    c[0] = c[2];
    c[2] = t;

    let pixel = Rgb(c);
    pixel
}
