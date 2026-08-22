//this is the CLI part for rendering and saving a rain GIF

use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};
use rand::Rng;
use std::fs::File;
use std::io::BufWriter;

struct Drop {
    x: f64,
    y: f64,
    len: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 400;
    let height = 300;
    let fps = 25;
    let seconds = 10;
    let count = 200;

    let mut rng = rand::thread_rng();
    let mut drops: Vec<Drop> = (0..count)
        .map(|_| Drop {
            x: rng.gen_range(0.0..width as f64),
            y: rng.gen_range(0.0..height as f64),
            len: rng.gen_range(8.0..20.0),
        })
        .collect();

    let out = BufWriter::new(File::create("rain.gif")?);
    let mut encoder = GifEncoder::new(out);
    encoder.set_repeat(Repeat::Infinite)?;

    let total_frames = fps * seconds;
    for _ in 0..total_frames {
        // step: move drops, recycle those leaving the bottom
        for d in drops.iter_mut() {
            d.y += 300.0 / fps as f64;
            if d.y > height as f64 + d.len {
                d.y = -d.len;
                d.x = rng.gen_range(0.0..width as f64);
            }
        }

        // render frame
        let mut img = RgbaImage::new(width, height);
        for p in img.pixels_mut() {
            *p = Rgba([20, 20, 35, 255]); // Nachthimmel
        }
        for d in &drops {
            draw_line(
                &mut img,
                d.x,
                d.y,
                d.x - 2.0,
                d.y - d.len,
                Rgba([160, 200, 255, 255]),
            );
        }

        encoder.encode_frame(Frame::from_parts(
            img,
            0,
            0,
            Delay::from_numer_denom_ms(1000 / fps, 1),
        ))?;
    }

    println!(
        "saved rain.gif ({} seconds, {} fps, {} frames)",
        seconds, fps, total_frames
    );
    Ok(())
}

// Bresenham line drawing
fn draw_line(img: &mut RgbaImage, x0: f64, y0: f64, x1: f64, y1: f64, color: Rgba<u8>) {
    let (mut x, mut y) = (x0.round() as i32, y0.round() as i32);
    let (x1, y1) = (x1.round() as i32, y1.round() as i32);
    let dx = (x1 - x).abs();
    let sx = if x < x1 { 1 } else { -1 };
    let dy = -(y1 - y).abs();
    let sy = if y < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}