import {
    default as wasm,
    Chip8Neo
} from "./chip_ahoyto.js";

const PIXEL_SET_COLOR = 0x50cb93ff;
const PIXEL_UNSET_COLOR = 0x1b1a17ff;

let LOGIC_HZ = 480;
const TIMER_HZ = 60;
const VISUAL_HZ = 60;

const KEYS = {
    "1": 0x01,
    "2": 0x02,
    "3": 0x03,
    "4": 0x0c,
    "q": 0x04,
    "w": 0x05,
    "e": 0x06,
    "r": 0x0d,
    "a": 0x07,
    "s": 0x08,
    "d": 0x09,
    "f": 0x0e,
    "z": 0x0a,
    "x": 0x00,
    "c": 0x0b,
    "v": 0x0f
}

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

    // initializes the complete set of sub-systems
    // and registers the event handlers
    init();
    register();

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
        
        // waits a little bit for the next frame to be draw
        // @todo need to define target time for draw
        await new Promise((resolve, reject) => {
            setTimeout(resolve, 1000 / VISUAL_HZ);
        })
    }
})();

const register = () => {
    registerDrop();
    registerKeys();
}

const registerDrop = () => {
    document.addEventListener("drop", async (event) => {
        event.preventDefault();
        event.stopPropagation();

        if (!event.dataTransfer.files) return;

        const file = event.dataTransfer.files[0];

        const arrayBuffer = await file.arrayBuffer();
        const data = new Uint8Array(arrayBuffer);

        state.chip8.reset_hard_ws();
        state.chip8.load_rom_ws(data);
    });
    document.addEventListener("dragover", async (event) => {
        event.preventDefault();
        event.stopPropagation();

        console.info("draging over");
    });
};

const registerKeys = () => {
    document.addEventListener("keydown", (event) => {
        const keyCode = KEYS[event.key];
        if (keyCode) {
            state.chip8.key_press_ws(keyCode);
            return;
        }

        switch(event.key) {
            case "+":
                LOGIC_HZ += 60;
                break;

            case "-":
                LOGIC_HZ += 60;
                break;
        }
    });

    document.addEventListener("keyup", (event) => {
        const keyCode = KEYS[event.key];
        if (keyCode) {
            state.chip8.key_lift_ws(keyCode);
            return;
        }
    });
}

const init = () => {
    initCanvas();
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
};

const updateCanvas = (pixels) => {
    for (let i = 0; i < pixels.length; i++) {
        state.videoBuff.setUint32(i * 4, pixels[i] ? PIXEL_SET_COLOR : PIXEL_UNSET_COLOR);
    }
    state.canvasCtx.putImageData(state.image, 0, 0);
    state.canvasScaledCtx.drawImage(state.canvas, 0, 0);
};
