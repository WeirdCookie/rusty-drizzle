
import init, { Rain } from './pkg/rusty_drizzle.js';


const canvas = document.getElementById('noise-canvas');
const ctx = canvas.getContext('2d');
let rain;

const sizeSlider = document.getElementById('size');
const speedSlider = document.getElementById('speed');
const countSlider = document.getElementById('count');
const angleSlider = document.getElementById('angle');
const widthSlider = document.getElementById('width');


function frame(ts) {
    const dt = Math.min((ts - (frame.last ?? ts)) / 1000, 0.05);
    rain.step(dt, canvas.width, canvas.height);
    rain.draw(ctx, canvas.width, canvas.height);
    frame.last = ts;
    requestAnimationFrame(frame);
}


async function start() {
    await init();
    rain = new Rain(Number(countSlider.value), canvas.width, canvas.height);
    rain.set_size(Number(sizeSlider.value));
    rain.set_speed(Number(speedSlider.value));
    rain.set_angle_deg(Number(angleSlider.value));
    rain.set_width(Number(widthSlider.value));
    requestAnimationFrame(frame);
}


sizeSlider.addEventListener('input', () => {
    document.getElementById('size-val').textContent = sizeSlider.value;
    rain.set_size(Number(sizeSlider.value));
});

speedSlider.addEventListener('input', () => {
    document.getElementById('speed-val').textContent = speedSlider.value;
    rain.set_speed(Number(speedSlider.value));
});

countSlider.addEventListener('input', () => {
    document.getElementById('count-val').textContent = countSlider.value;
    rain.set_count(Number(countSlider.value), canvas.width, canvas.height);
});

angleSlider.addEventListener('input', () => {
    document.getElementById('angle-val').textContent = angleSlider.value + '°';
    rain.set_angle_deg(Number(angleSlider.value));
});

widthSlider.addEventListener('input', () => {
    document.getElementById('width-val').textContent = widthSlider.value;
    rain.set_width(Number(widthSlider.value));
});


start();