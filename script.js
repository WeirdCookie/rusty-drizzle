
import init, { draw_noise } from './pkg/rusty_drizzle.js';


const canvas = document.getElementById('noise-canvas');
const ctx = canvas.getContext('2d');
const refreshButton = document.getElementById('refresh-btn');


function drawNoise() {
    draw_noise(ctx, canvas.width, canvas.height);
}


async function start() {
    await init();
    drawNoise();
    refreshButton.addEventListener('click', drawNoise);
}


start();