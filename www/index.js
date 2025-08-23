import init, { wasm_main } from "./pkg/embedded_engine.js";

async function run() {
    await init();
    wasm_main();
}

run().catch(console.error);
