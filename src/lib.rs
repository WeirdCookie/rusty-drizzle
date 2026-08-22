//this is the webassembly part for drawing the rain on the canvas

use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Rain {
    x: Vec<f64>,
    y: Vec<f64>,
    len: Vec<f64>,
}

#[wasm_bindgen]
impl Rain {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize) -> Rain {
        let mut rng = rand::thread_rng();
        Rain {
            x: (0..count).map(|_| rng.gen_range(0.0..640.0)).collect(),
            y: (0..count).map(|_| rng.gen_range(0.0..480.0)).collect(),
            len: (0..count).map(|_| rng.gen_range(8.0..20.0)).collect(),
        }
    }

    // moves all drops; those leaving the bottom respawn at the top
    pub fn step(&mut self, dt: f64, w: f64, h: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.y.len() {
            self.y[i] += 300.0 * dt;
            if self.y[i] > h + self.len[i] {
                self.y[i] = -self.len[i];
                self.x[i] = rng.gen_range(0.0..w);
            }
        }
    }

    // draws all drops as slanted strokes
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        ctx.set_fill_style_str("rgb(20, 20, 35)");
        ctx.fill_rect(0.0, 0.0, w, h);
        ctx.set_stroke_style_str("rgba(160, 200, 255, 0.7)");
        ctx.begin_path();
        for i in 0..self.y.len() {
            ctx.move_to(self.x[i], self.y[i]);
            ctx.line_to(self.x[i] - 2.0, self.y[i] - self.len[i]);
        }
        ctx.stroke();
    }
}