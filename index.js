import init, { greet, add } from './pkg/ransom.js';

async function run() {
    await init();
    const message = greet("js line 5");
    document.body.innerHTML = "<p>Render me ! https://github.com/brian-chilin/ransOm</p><p>" + add(6, 7) + "</p>";
}

run();
