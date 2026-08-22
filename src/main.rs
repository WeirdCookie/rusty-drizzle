//this is the CLI part for rendering and saving a rain GIF

mod sim;

use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};
use std::fs::File;
use std::io::BufWriter;

struct Params {
    size: f64,
    speed: f64,
    count: usize,
    angle_deg: f64,
    width: f64,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            size: 12.0,
            speed: 300.0,
            count: 200,
            angle_deg: 15.0,
            width: 2.0,
        }
    }
}

fn print_help() {
    println!("rusty-drizzle - renders 10 seconds of rain as rain.gif");
    println!();
    println!("Options:");
    println!("  --size  <f64>   drop size (length in px), default 12");
    println!("  --speed <f64>   fall speed (px/s), default 300");
    println!("  --count <int>   number of drops, default 200");
    println!("  --angle <f64>   fall angle in degrees from vertical, default 15");
    println!("  --width <f64>   drop width (stroke thickness in px), default 2");
    println!("  --help          show this help");
    println!();
    println!("Example:");
    println!("  rusty-drizzle --size 20 --speed 500 --count 400 --angle 30 --width 3");
}

fn value_at(args: &[String], i: &mut usize) -> Result<String, String> {
    *i += 1;
    args.get(*i)
        .cloned()
        .ok_or_else(|| format!("missing value for {}", args[*i - 1]))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut p = Params::default();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            "--size" => p.size = value_at(&args, &mut i)?.parse()?,
            "--speed" => p.speed = value_at(&args, &mut i)?.parse()?,
            "--count" => p.count = value_at(&args, &mut i)?.parse()?,
            "--angle" => p.angle_deg = value_at(&args, &mut i)?.parse()?,
            "--width" => p.width = value_at(&args, &mut i)?.parse()?,
            other => return Err(format!("unknown option: {}", other).into()),
        }
        i += 1;
    }

    let width = 400;
    let height = 300;
    let fps = 25;
    let seconds = 10;

    let mut rain = sim::Rain::new(
        p.count,
        width as f64,
        height as f64,
        p.size,
        p.speed,
        p.angle_deg,
    );

    let out = BufWriter::new(File::create("rain.gif")?);
    let mut encoder = GifEncoder::new(out);
    encoder.set_repeat(Repeat::Infinite)?;

    let total_frames = fps * seconds;
    for _ in 0..total_frames {
        rain.step(1.0 / fps as f64, width as f64, height as f64);

        let mut img = RgbaImage::new(width, height);
        for px in img.pixels_mut() {
            *px = Rgba([4, 19, 59, 255]); // dark blue night sky
        }
        for d in &rain.drops {
            draw_line(
                &mut img,
                d.x,
                d.y,
                d.x - d.ex,
                d.y - d.ey,
                p.width,
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
        "saved rain.gif ({}s, {}fps, {} frames) — size={} speed={} count={} angle={} width={}",
        seconds, fps, total_frames, p.size, p.speed, p.count, p.angle_deg, p.width
    );
    Ok(())
}

// Bresenham line drawing with a square brush of the given width
fn draw_line(img: &mut RgbaImage, x0: f64, y0: f64, x1: f64, y1: f64, w: f64, color: Rgba<u8>) {
    let (mut x, mut y) = (x0.round() as i32, y0.round() as i32);
    let (x1, y1) = (x1.round() as i32, y1.round() as i32);
    let half = ((w - 1.0) / 2.0).floor() as i32;
    let dx = (x1 - x).abs();
    let sx = if x < x1 { 1 } else { -1 };
    let dy = -(y1 - y).abs();
    let sy = if y < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        for py in y - half..=y + half {
            for px in x - half..=x + half {
                if px >= 0 && py >= 0 && (px as u32) < img.width() && (py as u32) < img.height() {
                    img.put_pixel(px as u32, py as u32, color);
                }
            }
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