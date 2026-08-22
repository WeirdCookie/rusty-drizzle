//this is the webassembly part for drawing the rain on the canvas

mod sim;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct Rain {
    sim: sim::Rain,
    width: f64,
    drop_style: String,
}

#[wasm_bindgen]
impl Rain {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize, w: f64, h: f64) -> Rain {
        Rain {
            sim: sim::Rain::new(count, w, h, 12.0, 300.0, 15.0),
            width: 2.0,
            drop_style: "rgba(160, 200, 255, 0.7)".to_string(),
        }
    }

    pub fn set_size(&mut self, v: f64) {
        self.sim.set_size(v);
    }

    pub fn set_speed(&mut self, v: f64) {
        self.sim.set_speed(v);
    }

    pub fn set_angle_deg(&mut self, v: f64) {
        self.sim.set_angle_deg(v);
    }

    pub fn set_count(&mut self, v: usize, w: f64, _h: f64) {
        self.sim.set_count(v, w);
    }

    pub fn set_width(&mut self, v: f64) {
        self.width = v;
    }

    pub fn step(&mut self, dt: f64, w: f64, h: f64) {
        self.sim.step(dt, w, h);
    }

    // draws all drops as slanted strokes in a single batched path
    pub fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        ctx.set_fill_style_str("rgb(4, 19, 59)");
        ctx.fill_rect(0.0, 0.0, w, h);
        ctx.set_stroke_style_str(&self.drop_style);
        ctx.set_line_width(self.width);
        ctx.begin_path();
        for d in self.sim.drops.iter() {
            ctx.move_to(d.x, d.y);
            ctx.line_to(d.x - d.ex, d.y - d.ey);
        }
        ctx.stroke();
    }
}