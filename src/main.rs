//this is the CLI part for rendering and saving a rain GIF

mod sim;

use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, Rgba, RgbaImage};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::str::FromStr;

// output file format
#[derive(Clone, Copy, PartialEq)]
enum Format {
    Gif,
    Png,
}

struct Params {
    format: Format,
    pic_width: u32,
    pic_height: u32,
    size: f64,
    speed: f64,
    count: usize,
    angle_deg: f64,
    width: f64,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            format: Format::Gif,
            pic_width: 400,
            pic_height: 300,
            size: 12.0,
            speed: 300.0,
            count: 200,
            angle_deg: 15.0,
            width: 2.0,
        }
    }
}

// reads a number from the terminal; pressing Enter keeps the default
fn read_number<T: FromStr + std::fmt::Display>(
    prompt: &str,
    default: T,
) -> Result<T, Box<dyn std::error::Error>>
where
    T::Err: std::fmt::Display,
{
    loop {
        print!("{} (default {}): ", prompt, default);
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let t = line.trim();
        if t.is_empty() {
            return Ok(default);
        }
        match t.parse::<T>() {
            Ok(v) => return Ok(v),
            Err(e) => println!("Invalid number ({}). Try again.", e),
        }
    }
}

// asks which output format to use; pressing Enter keeps the default (gif)
fn read_format(prompt: &str) -> Result<Format, Box<dyn std::error::Error>> {
    loop {
        print!("{} (default gif): ", prompt);
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let t = line.trim().to_ascii_lowercase();
        match t.as_str() {
            "" => return Ok(Format::Gif),
            "gif" => return Ok(Format::Gif),
            "png" => return Ok(Format::Png),
            _ => println!("Invalid format. Use 'gif' or 'png'."),
        }
    }
}

// interactive mode: asks for every parameter and uses the answers
fn user_input() -> Result<Params, Box<dyn std::error::Error>> {
    let mut p = Params::default();
    p.format = read_format("Output format (gif or png)")?;
    p.pic_width = read_number("Picture width in px", p.pic_width)?;
    p.pic_height = read_number("Picture height in px", p.pic_height)?;
    p.size = read_number("Drop size in px", p.size)?;
    p.speed = read_number("Fall speed in px/s", p.speed)?;
    p.count = read_number("Number of drops", p.count)?;
    p.angle_deg = read_number("Fall angle in degrees", p.angle_deg)?;
    Ok(p)
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
    println!("Run with no arguments to set the parameters interactively.");
    println!("  (interactive mode first asks whether to save a GIF or a PNG)");
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
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut p = Params::default();
    if args.is_empty() {
        // no command-line arguments -> interactive mode
        p = user_input()?;
    } else {
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
    }

    let width = p.pic_width;
    let height = p.pic_height;
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

    // build a single rendered frame (advances the simulation one step)
    fn render_frame(
        rain: &mut sim::Rain,
        width: u32,
        height: u32,
        stroke: f64,
    ) -> RgbaImage {
        rain.step(1.0 / 25.0, width as f64, height as f64);

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
                stroke,
                Rgba([160, 200, 255, 255]),
            );
        }
        img
    }

    if p.format == Format::Png {
        let img = render_frame(&mut rain, width, height, p.width);
        img.save("rain.png")?;
        println!(
            "saved rain.png ({}x{}) — size={} speed={} count={} angle={} width={}",
            width, height, p.size, p.speed, p.count, p.angle_deg, p.width
        );
        return Ok(());
    }

    let out = BufWriter::new(File::create("rain.gif")?);
    let mut encoder = GifEncoder::new(out);
    encoder.set_repeat(Repeat::Infinite)?;

    let total_frames = fps * seconds;
    for _ in 0..total_frames {
        let img = render_frame(&mut rain, width, height, p.width);

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