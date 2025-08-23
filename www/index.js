import init, { wasm_main } from "./pkg/embedded_engine.js";

// Prevent arrow keys from scrolling the page
window.addEventListener("keydown", (event) => {
    switch (event.code) {
        case "ArrowUp":
        case "ArrowDown":
        case "Space":
            event.preventDefault(); // stop scrolling
            break;
    }
});

async function run() {
    await init();
    await wasm_main();
}

run().catch(console.error);
