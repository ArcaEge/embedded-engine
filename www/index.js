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
    // Show loading text on canvas until the wasm loads
    const canvas = document.getElementById("game-canvas");
    const ctx = canvas.getContext("2d");
    ctx.fillText("Loading web assembly, please wait...", 10, 20);

    await init();
    await wasm_main();
}

run().catch(console.error);
