import {
    default as wasm,
    Chip8Neo
} from "./chip_ahoyto.js";

const PIXEL_SET_COLOR = 0x50cb93ff;
const PIXEL_UNSET_COLOR = 0x1b1a17ff;

const LOGIC_HZ = 480;
const TIMER_HZ = 60;
const VISUAL_HZ = 60;

const DISPLAY_WIDTH = 64;
const DISPLAY_HEIGHT = 32;

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

const ROM_PATH = "res/roms/pong.ch8";

const ROM_NAME = "pong.ch8";

const state = {
    chip8: null,
    logicFrequency: LOGIC_HZ,
    canvas: null,
    canvasScaled: null,
    canvasCtx: null,
    canvasScaledCtx: null,
    image: null,
    videoBuff: null,
    toastTimeout: null
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
    // target u8 array buffer
    const response = await fetch(ROM_PATH);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    // updates the ROM information on display
    setRom(ROM_NAME, data.length);

    // creates the CHIP-8 instance and resets it
    state.chip8 = new Chip8Neo();
    state.chip8.reset_hard_ws();
    state.chip8.load_rom_ws(data);

    // runs the sequence as an infinite loop, running
    // the associated CPU cycles accordingly
    while (true) {
        const ratioLogic = state.logicFrequency / VISUAL_HZ;
        for (let i = 0; i < ratioLogic; i++) {
            state.chip8.clock_ws();
        }

        const ratioTimer = TIMER_HZ / VISUAL_HZ;
        for (let i = 0; i < ratioTimer; i++) {
            state.chip8.clock_dt_ws();
            state.chip8.clock_st_ws();
        }

        // updates the canvas object with the new
        // visual information coming in
        updateCanvas(state.chip8.vram_ws());

        // waits a little bit for the next frame to be draw
        // @todo NEED TO DEFINE A TARGET TIME
        await new Promise((resolve, reject) => {
            setTimeout(resolve, 1000 / VISUAL_HZ);
        });
    }
})();

const register = () => {
    registerDrop();
    registerKeys();
    registerButtons();
    registerToast();
};

const registerDrop = () => {
    document.addEventListener("drop", async (event) => {
        if (!event.dataTransfer.files || event.dataTransfer.files.length === 0) return;

        event.preventDefault();
        event.stopPropagation();

        const overlay = document.getElementById("overlay");
        overlay.classList.remove("visible");

        const file = event.dataTransfer.files[0];

        if (!file.name.endsWith(".ch8")) {
            showToast("This is probably not a CHIP-8 ROM file!", true);
            return;
        }

        const arrayBuffer = await file.arrayBuffer();
        const data = new Uint8Array(arrayBuffer);

        state.chip8.reset_hard_ws();
        state.chip8.load_rom_ws(data);

        setRom(file.name, file.size);

        showToast(`Loaded ${file.name} ROM successfully!`);
    });
    document.addEventListener("dragover", async (event) => {
        if (!event.dataTransfer.items || event.dataTransfer.items[0].type) return;

        event.preventDefault();

        const overlay = document.getElementById("overlay");
        overlay.classList.add("visible");
    });
    document.addEventListener("dragenter", async (event) => {
        if (!event.dataTransfer.items || event.dataTransfer.items[0].type) return;
        const overlay = document.getElementById("overlay");
        overlay.classList.add("visible");
    });
    document.addEventListener("dragleave", async (event) => {
        if (!event.dataTransfer.items || event.dataTransfer.items[0].type) return;
        const overlay = document.getElementById("overlay");
        overlay.classList.remove("visible");
    });
};

const registerKeys = () => {
    document.addEventListener("keydown", (event) => {
        const keyCode = KEYS[event.key];
        if (keyCode !== undefined) {
            state.chip8.key_press_ws(keyCode);
            return;
        }

        switch (event.key) {
            case "+":
                setLogicFrequency(state.logicFrequency + 60);
                break;

            case "-":
                setLogicFrequency(state.logicFrequency - 60);
                break;

            case "Escape":
                const chipCanvas = document.getElementById("chip-canvas");
                chipCanvas.classList.remove("fullscreen");
                break;
        }
    });

    document.addEventListener("keyup", (event) => {
        const keyCode = KEYS[event.key];
        if (keyCode !== undefined) {
            state.chip8.key_lift_ws(keyCode);
            return;
        }
    });
};

const registerButtons = () => {
    const logicFrequencyPlus = document.getElementById("logic-frequency-plus");
    logicFrequencyPlus.addEventListener("click", (event) => {
        setLogicFrequency(state.logicFrequency + 60);
    });

    const logicFrequencyMinus = document.getElementById("logic-frequency-minus");
    logicFrequencyMinus.addEventListener("click", (event) => {
        setLogicFrequency(state.logicFrequency - 60);
    });

    const buttonBenchmark = document.getElementById("button-benchmark");
    buttonBenchmark.addEventListener("click", (event) => {
        console.info("Going to benchmark stuff");
    });

    const buttonFullscreen = document.getElementById("button-fullscreen");
    buttonFullscreen.addEventListener("click", (event) => {
        const chipCanvas = document.getElementById("chip-canvas");
        chipCanvas.classList.add("fullscreen");
    });

    const buttonInformation = document.getElementById("button-information");
    buttonInformation.addEventListener("click", (event) => {
        const sectionDiag = document.getElementById("section-diag");
        const separatorDiag = document.getElementById("separator-diag");
        if (buttonInformation.classList.contains("enabled")) {
            sectionDiag.style.display = "none";
            separatorDiag.style.display = "none";
            buttonInformation.classList.remove("enabled");
        } else {
            sectionDiag.style.display = "block";
            separatorDiag.style.display = "block";
            buttonInformation.classList.add("enabled");
        }
    });
};

const registerToast = () => {
    const toast = document.getElementById("toast");
    toast.addEventListener("click", (event) => {
        toast.classList.remove("visible");
    });
};

const showToast = async (message, error = false, timeout = 3500) => {
    const toast = document.getElementById("toast");
    toast.classList.remove("error");
    if (error) toast.classList.add("error");
    toast.classList.add("visible");
    toast.textContent = message;
    if (state.toastTimeout) clearTimeout(state.toastTimeout);
    state.toastTimeout = setTimeout(() => {
        toast.classList.remove("visible");
        state.toastTimeout = null;
    }, timeout);
}

const setRom = (name, size) => {
    state.romName = name;
    state.romSize = size;
    document.getElementById("rom-name").textContent = name;
    document.getElementById("rom-size").textContent = String(size);
};

const setLogicFrequency = (value) => {
    if (value < 0) showToast("Invalid frequency value!", true);
    value = Math.max(value, 0);
    state.logicFrequency = value;
    document.getElementById("logic-frequency").textContent = value;
};

const init = () => {
    initCanvas();
};

const initCanvas = () => {
    // initializes the off-screen canvas that is going to be
    // used in the drawing process
    state.canvas = document.createElement("canvas");
    state.canvas.width = DISPLAY_WIDTH;
    state.canvas.height = DISPLAY_HEIGHT;
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