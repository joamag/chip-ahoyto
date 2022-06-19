import { default as wasm, Chip8Neo } from "./chip_ahoyto.js";

(async () => {
    // initializes the WASM module, this is required
    // so that the global symbols become available
    await wasm();

    const ROM = "roms/ibm_logo.ch8";

    console.info("LOADED");

    const response = await fetch(ROM);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    // creates the CHIP-8 instance and resets it
    const chip8 = new Chip8Neo();
    chip8.reset_hard_ws();
    chip8.load_rom_ws(data);

    console.info(`Loaded ${ROM}`);

    chip8.clock_ws();    
})();
