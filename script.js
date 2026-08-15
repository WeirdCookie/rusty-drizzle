
import init, { draw_noise } from './pkg/rusty_drizzle.js';


const canvas = document.getElementById('noise-canvas');
const ctx = canvas.getContext('2d');


function drawNoise() {
    draw_noise(ctx, canvas.width, canvas.height);
    requestAnimationFrame(drawNoise);
}


async function start() {
    await init();
    drawNoise();
}


start();