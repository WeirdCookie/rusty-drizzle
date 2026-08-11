use std::io::Cursor;
use image::ImageReader;

fn main() -> image::ImageResult<()> {
    let img = ImageReader::open("myimage.png")?.decode()?;
    let mut bytes: Vec<u8> = Vec::new();
    let img2 = ImageReader::new(Cursor::new(&bytes)).with_guessed_format()?.decode()?;
    img.save("empty.jpg")?;
    img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;
    Ok(())
}
