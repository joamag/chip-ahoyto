import {
    default as wasm,
    Chip8Neo
} from "./chip_ahoyto.js";

const PIXEL_SET_COLOR = 0x50cb93ff;
const PIXEL_UNSET_COLOR = 0x1b1a17ff;

const state = {
    canvas: null,
    canvasScaled: null,
    canvasCtx: null,
    canvasScaledCtx: null,
    image: null,
    videoBuff: null
};

(async () => {
    // initializes the WASM module, this is required
    // so that the global symbols become available
    await wasm();

    const ROM = "roms/ibm_logo.ch8";

    console.info("System Loaded!");

    // loads the ROM data and converts it into the
    // target u8 array bufffer
    const response = await fetch(ROM);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    // creates the CHIP-8 instance and resets it
    const chip8 = new Chip8Neo();
    chip8.reset_hard_ws();
    chip8.load_rom_ws(data);

    console.info(`ROM Loaded ${ROM}`);

    initCanvas();

    while (true) {
        chip8.clock_ws();
        updateCanvas(chip8.vram_ws());
        
        // hack
        await new Promise((resolve, reject) => {
            setTimeout(resolve, 100);
        })
    }
})();

const initCanvas = () => {
    // initializes the off-screen canvas that is going to be
    // used in the drawing proces
    state.canvas = document.createElement("canvas");
    state.canvas.width = 64;
    state.canvas.height = 32;
    state.canvasCtx = state.canvas.getContext("2d");

    state.canvasScaled = document.getElementById("chip-canvas");
    state.canvasScaledCtx = state.canvasScaled.getContext("2d");

    state.canvasScaledCtx.scale(state.canvasScaled.width / state.canvas.width, state.canvasScaled.height / state.canvas.height);
    state.canvasScaledCtx.imageSmoothingEnabled = false;

    state.image = state.canvasCtx.createImageData(state.canvas.width, state.canvas.height);
    state.videoBuff = new DataView(state.image.data.buffer);
}

const updateCanvas = (pixels) => {
    for (let i = 0; i < pixels.length; i++) {
        state.videoBuff.setUint32(i * 4, pixels[i] ? PIXEL_SET_COLOR : PIXEL_UNSET_COLOR);
    }
    state.canvasCtx.putImageData(state.image, 0, 0);
    state.canvasScaledCtx.drawImage(state.canvas, 0, 0);
}
