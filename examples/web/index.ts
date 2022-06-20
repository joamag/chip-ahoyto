import {
    default as wasm,
    Chip8Neo,
    Chip8Classic
} from "./lib/chip_ahoyto.js";

const PIXEL_SET_COLOR = 0x50cb93ff;
const PIXEL_UNSET_COLOR = 0x1b1a17ff;

const LOGIC_HZ = 600;
const VISUAL_HZ = 60;
const TIMER_HZ = 60;

const FREQUENCY_DELTA = 60

const DISPLAY_WIDTH = 64;
const DISPLAY_HEIGHT = 32;

const BACKGROUNDS = [
    "1b1a17",
    "023047",
    "bc6c25",
    "264653",
    "283618"
]

const KEYS: Record<string, number> = {
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

type State = {
    chip8: Chip8Neo | Chip8Classic,
    logicFrequency: number,
    visualFrequency: number,
    timerFrequency: number,
    canvas: HTMLCanvasElement,
    canvasScaled: HTMLCanvasElement,
    canvasCtx: CanvasRenderingContext2D,
    canvasScaledCtx: CanvasRenderingContext2D,
    image: ImageData,
    videoBuff: DataView,
    toastTimeout: number,
    paused: boolean,
    background_index: number,
    nextTickTime: number,
    fps: number,
    frameStart: number,
    frameCount: number,
    romName: string,
    romSize: number
};

const state: State = {
    chip8: null,
    logicFrequency: LOGIC_HZ,
    visualFrequency: VISUAL_HZ,
    timerFrequency: TIMER_HZ,
    canvas: null,
    canvasScaled: null,
    canvasCtx: null,
    canvasScaledCtx: null,
    image: null,
    videoBuff: null,
    toastTimeout: null,
    paused: false,
    background_index: 0,
    nextTickTime: 0,
    fps: VISUAL_HZ,
    frameStart: new Date().getTime(),
    frameCount: 0,
    romName: null,
    romSize: 0
};

(async () => {
    // initializes the WASM module, this is required
    // so that the global symbols become available
    await wasm();

    // initializes the complete set of sub-systems
    // and registers the event handlers
    await init();
    await register();

    // start the emulator subsystem with the initial
    // ROM retrieved from a remote data source
    await start();

    // runs the sequence as an infinite loop, running
    // the associated CPU cycles accordingly
    while (true) {
        if (state.paused) {
            await new Promise((resolve) => {
                setTimeout(resolve, 100);
            });
            continue;
        }

        let currentTime = new Date().getTime();

        // in case the time to draw the next frame has been
        // reached the flush of the logic and visuals is done
        if (currentTime >= state.nextTickTime) {
            // calculates the number of ticks that have elapsed since the
            // last draw operation, this is critical to be able to properly
            // operate the clock of the CPU in frame drop situations
            if (state.nextTickTime === 0) state.nextTickTime = currentTime;
            let ticks = Math.ceil((currentTime - state.nextTickTime) / (1 / state.visualFrequency * 1000));
            ticks = Math.max(ticks, 1);

            const ratioLogic = state.logicFrequency / state.visualFrequency * ticks;
            for (let i = 0; i < ratioLogic; i++) {
                state.chip8.clock_ws();
            }

            const ratioTimer = state.timerFrequency / state.visualFrequency * ticks;
            for (let i = 0; i < ratioTimer; i++) {
                state.chip8.clock_dt_ws();
                state.chip8.clock_st_ws();
            }

            // updates the canvas object with the new
            // visual information coming in
            updateCanvas(state.chip8.vram_ws());

            // increments the number of frames rendered in the current
            // section, this value is going to be used to calculate FPS
            state.frameCount += 1;

            // in case the target number of frames for FPS control
            // has been reached calculates the number of FPS and
            // flushes the value to the screen
            if (state.frameCount === state.visualFrequency * 2) {
                const currentTime = new Date().getTime();
                const deltaTime = (currentTime - state.frameStart) / 1000;
                const fps = Math.round(state.frameCount / deltaTime) ;
                setFps(fps);
                state.frameCount = 0;
                state.frameStart = currentTime;
            }

            // updates the next update time reference to the, so that it
            // can be used to control the game loop
            state.nextTickTime += 1000 / state.visualFrequency * ticks;
        }

        // calculates the amount of time until the next draw operation
        // this is the amount of time that is going to be pending
        currentTime = new Date().getTime();
        const pendingTime = Math.max(state.nextTickTime - currentTime, 0);

        // waits a little bit for the next frame to be draw,
        // this should control the flow of render
        await new Promise((resolve) => {
            setTimeout(resolve, pendingTime);
        });
    }
})();

const start = async ({ romPath = ROM_PATH, engine = "neo" } = {} ) => {
    // extracts the name of the ROM from the provided
    // path by splitting its structure
    const romPathS = romPath.split(/\//g);
    const romName = romPathS[romPathS.length - 1];

    // loads the ROM data and converts it into the
    // target byte array buffer (to be used by WASM)
    const response = await fetch(ROM_PATH);
    const blob = await response.blob();
    const arrayBuffer = await blob.arrayBuffer();
    const data = new Uint8Array(arrayBuffer);

    // updates the ROM information on display
    setRom(romName, data.length);
    setLogicFrequency(state.logicFrequency);
    setFps(state.fps);

    // selects the proper engine for execution
    // and builds a new instance of it
    switch(engine) {
        case "neo":
            state.chip8 = new Chip8Neo();
            break;

        case "classic":
            state.chip8 = new Chip8Classic();
            break;
    }

    // resets the CHIP-8 engine to restore it into
    // a valid state ready to be used
    state.chip8.reset_hard_ws();
    state.chip8.load_rom_ws(data);
}

const register = async () => {
    registerDrop();
    registerKeys();
    registerButtons();
    registerToast();
};

const init = async () => {
    initCanvas();
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
                setLogicFrequency(state.logicFrequency + FREQUENCY_DELTA);
                break;

            case "-":
                setLogicFrequency(state.logicFrequency - FREQUENCY_DELTA);
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
    logicFrequencyPlus.addEventListener("click", () => {
        setLogicFrequency(state.logicFrequency + FREQUENCY_DELTA);
    });

    const logicFrequencyMinus = document.getElementById("logic-frequency-minus");
    logicFrequencyMinus.addEventListener("click", () => {
        setLogicFrequency(state.logicFrequency - FREQUENCY_DELTA);
    });

    const buttonPause = document.getElementById("button-pause");
    buttonPause.addEventListener("click", () => {
        toggleRunning();
    });

    const buttonBenchmark = document.getElementById("button-benchmark");
    buttonBenchmark.addEventListener("click", () => {
        buttonBenchmark.classList.add("enabled");
        pause();
        try {
            const initial = Date.now();
            const count = 500000000;
            for (let i = 0; i < count; i++) {
                state.chip8.clock_ws();
            }
            const delta = (Date.now() - initial) / 1000;
            const frequency_mhz = count / delta / 1000 / 1000;
            showToast(`Took ${delta.toFixed(2)} seconds to run ${count} ticks (${frequency_mhz.toFixed(2)} Mhz)!`, undefined, 7500);
        } finally {
            resume();
            buttonBenchmark.classList.remove("enabled");
        }
    });

    const buttonFullscreen = document.getElementById("button-fullscreen");
    buttonFullscreen.addEventListener("click", () => {
        const chipCanvas = document.getElementById("chip-canvas");
        chipCanvas.classList.add("fullscreen");
    });

    const buttonInformation = document.getElementById("button-information");
    buttonInformation.addEventListener("click", () => {
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

    const buttonTheme = document.getElementById("button-theme");
    buttonTheme.addEventListener("click", () => {
        state.background_index = (state.background_index + 1) % BACKGROUNDS.length;
        const background = BACKGROUNDS[state.background_index];
        document.body.style.backgroundColor = `#${background}`;
        document.getElementById("footer").style.backgroundColor = `#${background}`;
    });
};

const registerToast = () => {
    const toast = document.getElementById("toast");
    toast.addEventListener("click", (event) => {
        toast.classList.remove("visible");
    });
};


const initCanvas = () => {
    // initializes the off-screen canvas that is going to be
    // used in the drawing process
    state.canvas = document.createElement("canvas");
    state.canvas.width = DISPLAY_WIDTH;
    state.canvas.height = DISPLAY_HEIGHT;
    state.canvasCtx = state.canvas.getContext("2d");

    state.canvasScaled = document.getElementById("chip-canvas") as HTMLCanvasElement;
    state.canvasScaledCtx = state.canvasScaled.getContext("2d");

    state.canvasScaledCtx.scale(state.canvasScaled.width / state.canvas.width, state.canvasScaled.height / state.canvas.height);
    state.canvasScaledCtx.imageSmoothingEnabled = false;

    state.image = state.canvasCtx.createImageData(state.canvas.width, state.canvas.height);
    state.videoBuff = new DataView(state.image.data.buffer);
};

const updateCanvas = (pixels: Uint8Array) => {
    for (let i = 0; i < pixels.length; i++) {
        state.videoBuff.setUint32(i * 4, pixels[i] ? PIXEL_SET_COLOR : PIXEL_UNSET_COLOR);
    }
    state.canvasCtx.putImageData(state.image, 0, 0);
    state.canvasScaledCtx.drawImage(state.canvas, 0, 0);
};

const showToast = async (message: string, error = false, timeout = 3500) => {
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

const setRom = (name: string, size: number) => {
    state.romName = name;
    state.romSize = size;
    document.getElementById("rom-name").textContent = name;
    document.getElementById("rom-size").textContent = String(size);
};

const setLogicFrequency = (value: number) => {
    if (value < 0) showToast("Invalid frequency value!", true);
    value = Math.max(value, 0);
    state.logicFrequency = value;
    document.getElementById("logic-frequency").textContent = String(value);
};

const setFps = (value: number) => {
    if (value < 0) showToast("Invalid FPS value!", true);
    value = Math.max(value, 0);
    state.fps = value;
    document.getElementById("fps-count").textContent = String(value);
};

const toggleRunning = () => {
    const buttonPause = document.getElementById("button-pause");
    if (buttonPause.textContent === "Resume") {
        resume();
    } else {
        pause();
    }
};

const pause = () => {
    state.paused = true;
    const buttonPause = document.getElementById("button-pause");
    buttonPause.classList.add("enabled");
    buttonPause.textContent = "Resume";
}

const resume = () => {
    state.paused = false;
    state.nextTickTime = new Date().getTime();
    const buttonPause = document.getElementById("button-pause");
    buttonPause.classList.remove("enabled");
    buttonPause.textContent = "Pause";
}
