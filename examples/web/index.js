import {
    default as wasm,
    Chip8Neo
} from "./chip_ahoyto.js";

const PIXEL_SET_COLOR = 0x50cb93ff;
const PIXEL_UNSET_COLOR = 0x1b1a17ff;

const LOGIC_HZ = 480;
const TIMER_HZ = 60;
const VISUAL_HZ = 60;

const ROM = "res/roms/pong.ch8";

const state = {
    chip8: null,
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

    // initializes the canvas sub-sytem
    initCanvas();
    registerDrop();

    // loads the ROM data and converts it into the
    // target u8 array bufffer
    const response = await fetch(ROM);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    // creates the CHIP-8 instance and resets it
    state.chip8 = new Chip8Neo();
    state.chip8.reset_hard_ws();
    state.chip8.load_rom_ws(data);

    // runs the sequence as an infinite loop, running
    // the associated CPU cycles accordingly
    while (true) {        
        const ratioLogic = LOGIC_HZ / VISUAL_HZ;
        for(let i = 0; i < ratioLogic; i++) {
            state.chip8.clock_ws();
        }

        const ratioTimer = TIMER_HZ / VISUAL_HZ;
        for(let i = 0; i < ratioTimer; i++) {
            state.chip8.clock_dt_ws();
            state.chip8.clock_st_ws();
        }

        // updates the canvas object with the new
        // visual information comming in
        updateCanvas(state.chip8.vram_ws());
        
        // @todo hack
        await new Promise((resolve, reject) => {
            setTimeout(resolve, 1000 / VISUAL_HZ);
        })
    }
})();

const registerDrop = () => {
    document.addEventListener("drop", async (e) => {
        e.preventDefault();
        e.stopPropagation();

        if (!e.dataTransfer.files) return;

        const file = e.dataTransfer.files[0];

        const arrayBuffer = await file.arrayBuffer();
        const data = new Uint8Array(arrayBuffer);

        state.chip8.reset_hard_ws();
        state.chip8.load_rom_ws(data);
    });
    document.addEventListener("dragover", async (e) => {
        e.preventDefault();
        e.stopPropagation();

        console.info("draging over");
    });
}

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
