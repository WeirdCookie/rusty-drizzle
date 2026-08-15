//this is the webassembly part for drawing on the canvas

use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn draw_noise(ctx: &CanvasRenderingContext2d, width: u32, height: u32) {
    ctx.set_fill_style(&"rgb(30, 30, 40)".into());
    ctx.fill_rect(0.0, 0.0, width.into(), height.into());

    let mut rng = rand::thread_rng();

    for x in 0..width {
        for y in 0..height {
            let r: u8 = rng.gen();
            let g: u8 = rng.gen();
            let b: u8 = rng.gen();

            ctx.set_fill_style(&format!("rgb({},{},{})", r, g, b).into());
            ctx.fill_rect(x.into(), y.into(), 1.0, 1.0);
        }
    }
}