
import init, { Rain } from './pkg/rusty_drizzle.js';


const canvas = document.getElementById('noise-canvas');
const ctx = canvas.getContext('2d');
let rain;


function frame(ts) {
    const dt = (ts - (frame.last ?? ts)) / 1000;
    rain.step(dt, canvas.width, canvas.height);
    rain.draw(ctx, canvas.width, canvas.height);
    frame.last = ts;
    requestAnimationFrame(frame);
}


async function start() {
    await init();
    rain = new Rain(300);
    requestAnimationFrame(frame);
}


start();