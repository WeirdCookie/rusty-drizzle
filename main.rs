use image::{ImageBuffer, Rgb};
use rand::Rng;

fn main() {
    let width = 400;
    let height = 300;
    let mut img = ImageBuffer::new(width, height);

    
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();

        *pixel = Rgb([r, g, b]);
    }

   
    img.save("random_pattern.png").unwrap();

    println!("saved picture random_pattern.png");
}