import init, { greet, add, generate_image  } from './pkg/ransom.js';

async function run() {
    await init();

    const message = greet("js line 6");
    document.body.innerHTML += "<p>https://github.com/brian-chilin/ransOm</p><p>" + add(6, 7) + "</p>";

    const width = 40;
    const height = 8;

    const pixels = generate_image(width, height); // Uint8Array in JS

    const canvas = document.getElementById('canvas');
    canvas.width = width;
    canvas.height = height;
    const ctx = canvas.getContext('2d');

    // Wrap raw pixels in ImageData
    const imageData = new ImageData(new Uint8ClampedArray(pixels), width, height);
    ctx.putImageData(imageData, 0, 0);
}

run();
