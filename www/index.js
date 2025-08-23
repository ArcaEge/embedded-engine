import init, { wasm_main } from "./pkg/embedded_engine.js";

// Prevent arrow keys from scrolling the page
window.addEventListener("keydown", (event) => {
    event.preventDefault();
});

async function run() {
    await init();
    await wasm_main();
}

run().catch(console.error);
