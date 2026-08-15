
import init, { draw_noise } from './pkg/rusty_drizzle.js';


const canvas = document.getElementById('noise-canvas');
const ctx = canvas.getContext('2d');


async function start() {
    await init();                
    draw_noise(ctx, 640, 480);   
}


start();