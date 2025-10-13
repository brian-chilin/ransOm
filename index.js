import init, { greet } from './pkg/ransom.js';

async function run() {
    await init();
    const message = greet("js line 5");
    document.body.textContent = "Render me ! https://github.com/brian-chilin/ransOm";
}

run();
