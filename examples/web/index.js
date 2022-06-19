import { default as wasm, Chip8Neo } from "./chip_ahoyto.js";

(async () => {
    // initializes the WASM module, this is required
    // so that the global symbols become available
    await wasm();

    console.info("LOADED");

    const chip8 = new Chip8Neo();
    chip8.clock_ws();

    console.info("CLOCK");
})();
