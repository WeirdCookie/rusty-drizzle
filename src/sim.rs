//! Shared rain simulation, used by the wasm browser renderer and the CLI GIF renderer.
//!
//! Performance notes:
//! - velocity (vx, vy) and the streak offset (ex, ey) are precomputed once per drop,
//!   so `step` only does two additions per drop (no trig, no sqrt per frame).
//! - the RNG is stored in the struct, avoiding a `thread_rng()` lookup per call.

use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Drop {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub ex: f64,
    pub ey: f64,
    pub len: f64,
}

pub struct Rain {
    pub drops: Vec<Drop>,
    rng: ThreadRng,
    size: f64,
    speed: f64,
    angle_deg: f64,
}

fn random_len(rng: &mut ThreadRng, size: f64) -> f64 {
    size * (0.6 + rng.gen::<f64>() * 0.8)
}

fn apply_velocity(rng: &mut ThreadRng, d: &mut Drop, speed: f64, angle_deg: f64) {
    let sp = speed * (0.7 + rng.gen::<f64>() * 0.6);
    let ang = (angle_deg + (rng.gen::<f64>() - 0.5) * 12.0).to_radians();
    d.vx = ang.sin() * sp;
    d.vy = ang.cos() * sp;
    let inv = 1.0 / (d.vx * d.vx + d.vy * d.vy).sqrt();
    d.ex = d.vx * inv * d.len;
    d.ey = d.vy * inv * d.len;
}

fn respawn(rng: &mut ThreadRng, d: &mut Drop, w: f64, size: f64, speed: f64, angle_deg: f64) {
    d.len = random_len(rng, size);
    d.x = rng.gen_range(0.0..w);
    d.y = -rng.gen_range(0.0..d.len * 2.0) - d.len;
    apply_velocity(rng, d, speed, angle_deg);
}

// Setters are used by the wasm browser renderer (lib.rs), not by the CLI bin.
#[allow(dead_code)]
impl Rain {
    pub fn new(count: usize, w: f64, h: f64, size: f64, speed: f64, angle_deg: f64) -> Rain {
        let rng = rand::thread_rng();
        let mut rain = Rain {
            drops: Vec::with_capacity(count),
            rng,
            size,
            speed,
            angle_deg,
        };
        for _ in 0..count {
            let mut d = Drop {
                x: 0.0,
                y: 0.0,
                vx: 0.0,
                vy: 0.0,
                ex: 0.0,
                ey: 0.0,
                len: 0.0,
            };
            d.len = random_len(&mut rain.rng, size);
            d.x = rain.rng.gen_range(0.0..w);
            d.y = rain.rng.gen_range(0.0..h);
            apply_velocity(&mut rain.rng, &mut d, speed, angle_deg);
            rain.drops.push(d);
        }
        rain
    }

    pub fn set_size(&mut self, v: f64) {
        self.size = v;
        self.reapply_velocity();
    }

    pub fn set_speed(&mut self, v: f64) {
        self.speed = v;
        self.reapply_velocity();
    }

    pub fn set_angle_deg(&mut self, v: f64) {
        self.angle_deg = v;
        self.reapply_velocity();
    }

    pub fn set_count(&mut self, count: usize, w: f64) {
        self.drops.truncate(count);
        let (size, speed, angle_deg) = (self.size, self.speed, self.angle_deg);
        while self.drops.len() < count {
            let mut d = Drop {
                x: 0.0,
                y: 0.0,
                vx: 0.0,
                vy: 0.0,
                ex: 0.0,
                ey: 0.0,
                len: 0.0,
            };
            respawn(&mut self.rng, &mut d, w, size, speed, angle_deg);
            self.drops.push(d);
        }
    }

    pub fn step(&mut self, dt: f64, w: f64, h: f64) {
        let rng = &mut self.rng;
        let (size, speed, angle_deg) = (self.size, self.speed, self.angle_deg);
        for d in self.drops.iter_mut() {
            d.x += d.vx * dt;
            d.y += d.vy * dt;
            if d.y > h + d.len || d.x < -d.len || d.x > w + d.len {
                respawn(rng, d, w, size, speed, angle_deg);
            }
        }
    }

    fn reapply_velocity(&mut self) {
        let speed = self.speed;
        let angle_deg = self.angle_deg;
        for d in self.drops.iter_mut() {
            apply_velocity(&mut self.rng, d, speed, angle_deg);
        }
    }
}
