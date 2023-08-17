import {
    BenchmarkResult,
    Emulator,
    EmulatorBase,
    Entry,
    Feature,
    Frequency,
    FrequencySpecs,
    PixelFormat,
    RomInfo,
    SaveState,
    Size
} from "emukit";
import { PALETTES, PALETTES_MAP } from "./palettes";

import {
    default as wasm,
    Chip8Neo,
    Chip8Classic,
    Info
} from "../lib/chip_ahoyto";
import info from "../package.json";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
declare const require: any;

const LOGIC_HZ = 600;
const VISUAL_HZ = 60;
const TIMER_HZ = 60;
const IDLE_HZ = 10;

const DISPLAY_WIDTH = 64;
const DISPLAY_HEIGHT = 32;
const DISPLAY_SCALE = 8;

/**
 * The sample rate that is going to be used for FPS calculus,
 * meaning that every N seconds we will calculate the number
 * of frames rendered divided by the N seconds.
 */
const FPS_SAMPLE_RATE = 3;

const SOUND_DATA =
    "data:audio/mpeg;base64,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AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP/7ksD/gAAAASwAAAAAAAAlgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD/+5LA/4AAAAEsAAAAAAAAJYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA//uSwP+AAAABLAAAAAAAACWAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP/7ksD/gAAAASwAAAAAAAAlgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD/+5LA/4AAAAEsAAAAAAAAJYAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA//uSwP+AAAABLAAAAAAAACWAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP/7ksD/gAAAASwAAAAAAAAlgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAD/+5LA/4AAAAEsAAAAAAAAJYAAAAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAgACAAIAAA";

const KEYS: Record<string, number> = {
    "1": 0x01,
    "2": 0x02,
    "3": 0x03,
    "4": 0x0c,
    Q: 0x04,
    W: 0x05,
    E: 0x06,
    R: 0x0d,
    A: 0x07,
    S: 0x08,
    D: 0x09,
    F: 0x0e,
    Z: 0x0a,
    X: 0x00,
    C: 0x0b,
    V: 0x0f
};

const ROM_PATH = require("../../../res/roms/pong.ch8");

const sound = ((data = SOUND_DATA, volume = 0.2) => {
    const sound = new Audio(data);
    sound.volume = volume;
    sound.muted = true;
    return sound;
})();

export class Chip8Emulator extends EmulatorBase implements Emulator {
    /**
     * The CHIP-8 engine (probably coming from WASM) that
     * is going to be used for the emulation.
     */
    private chip8: Chip8Neo | Chip8Classic | null = null;

    /**
     * The descriptive name of the engine that is currently
     * in use to emulate the system.
     */
    private _engine: string | null = null;

    private logicFrequency = LOGIC_HZ;
    private visualFrequency = VISUAL_HZ;
    private idleFrequency = IDLE_HZ;
    private timerFrequency = TIMER_HZ;

    private paused = false;
    private nextTickTime = 0;
    private fps = 0;
    private frameStart = EmulatorBase.now();
    private frameCount = 0;
    private paletteIndex = 0;

    private romName: string | null = null;
    private romData: Uint8Array | null = null;
    private romSize = 0;

    async init() {
        // initializes the WASM module, this is required
        // so that the global symbols become available
        await wasm();
    }

    async main({ romUrl }: { romUrl?: string }) {
        // boots the emulator subsystem with the initial
        // ROM retrieved from a remote data source
        await this.boot({ loadRom: true, romPath: romUrl ?? undefined });

        // runs the sequence as an infinite loop, running
        // the associated CPU cycles accordingly
        while (true) {
            // in case the machine is paused we must delay the execution
            // a little bit until the paused state is recovered
            if (this.paused) {
                await new Promise((resolve) => {
                    setTimeout(resolve, 1000 / this.idleFrequency);
                });
                continue;
            }

            // obtains the current time, this value is going
            // to be used to compute the need for tick computation
            let currentTime = EmulatorBase.now();

            try {
                this.tick(currentTime);
            } catch (err) {
                // sets the default error message to be displayed
                // to the user
                let message = String(err);

                // verifies if the current issue is a panic one
                // and updates the message value if that's the case
                const messageNormalized = (err as Error).message.toLowerCase();
                const isPanic =
                    messageNormalized.startsWith("unreachable") ||
                    messageNormalized.startsWith("recursive use of an object");
                if (isPanic) {
                    message = "Unrecoverable error, restarting CHIP-8";
                }

                // displays the error information to both the end-user
                // and the developer (for diagnostics)
                this.trigger("message", {
                    text: message,
                    error: true,
                    timeout: 5000
                });
                console.error(err);

                // pauses the machine, allowing the end-user to act
                // on the error in a proper fashion
                this.pause();

                // if we're talking about a panic proper action must be taken
                // which in this case it means restarting both the WASM sub
                // system and the machine state (to be able to recover)
                // also sets the default color on screen to indicate the issue
                if (isPanic) {
                    await wasm();
                    await this.boot({ restore: false });

                    this.trigger("error");
                }
            }

            // calculates the amount of time until the next draw operation
            // this is the amount of time that is going to be pending
            currentTime = EmulatorBase.now();
            const pendingTime = Math.max(this.nextTickTime - currentTime, 0);

            // waits a little bit for the next frame to be draw,
            // this should control the flow of render
            await new Promise((resolve) => {
                setTimeout(resolve, pendingTime);
            });
        }
    }

    tick(currentTime: number) {
        // in case the time to draw the next frame has not been
        // reached the flush of the "tick" logic is skiped
        if (currentTime < this.nextTickTime) return;

        // initializes the flag that is going to control is a beep
        // is going to be issued
        let beepFlag = false;

        // calculates the number of ticks that have elapsed since the
        // last draw operation, this is critical to be able to properly
        // operate the clock of the CPU in frame drop situations
        if (this.nextTickTime === 0) this.nextTickTime = currentTime;
        let ticks = Math.ceil(
            (currentTime - this.nextTickTime) /
                ((1 / this.visualFrequency) * 1000)
        );
        ticks = Math.max(ticks, 1);

        const ratioLogic = (this.logicFrequency / this.visualFrequency) * ticks;
        for (let i = 0; i < ratioLogic; i++) {
            this.chip8?.clock_ws();
        }

        const ratioTimer = (this.timerFrequency / this.visualFrequency) * ticks;
        for (let i = 0; i < ratioTimer; i++) {
            this.chip8?.clock_dt_ws();
            this.chip8?.clock_st_ws();
            beepFlag ||= this.chip8?.beep_ws() ?? false;
        }

        // in case the beep flag is active issue a sound during a bried
        // period, to notify the user about a certain event
        if (beepFlag) beep();

        // triggers the frame event indicating that
        // a new frame is now available for drawing
        this.trigger("frame");

        // marks the vertical blank interrupt effectively indicating
        // that a new frame can be drawn from a logical point of view
        this.chip8?.vblank_ws();

        // increments the number of frames rendered in the current
        // section, this value is going to be used to calculate FPS
        this.frameCount += 1;

        // in case the target number of frames for FPS control
        // has been reached calculates the number of FPS and
        // flushes the value to the screen
        if (this.frameCount === this.visualFrequency * FPS_SAMPLE_RATE) {
            const currentTime = EmulatorBase.now();
            const deltaTime = (currentTime - this.frameStart) / 1000;
            const fps = Math.round(this.frameCount / deltaTime);
            this.fps = fps;
            this.frameCount = 0;
            this.frameStart = currentTime;
        }

        // updates the next update time reference to the, so that it
        // can be used to control the game loop
        this.nextTickTime += (1000 / this.visualFrequency) * ticks;
    }

    /**
     * Starts the current machine, setting the internal structure in
     * a proper state to start drawing and receiving input.
     *
     * This method can also be used to load a new ROM into the machine.
     *
     * @param options The options that are going to be used in the
     * starting of the machine, includes information on the ROM and
     * the emulator engine to use.
     */
    async boot({
        engine = "neo",
        restore = true,
        loadRom = false,
        romPath = ROM_PATH,
        romName = null,
        romData = null
    }: {
        engine?: string | null;
        restore?: boolean;
        loadRom?: boolean;
        romPath?: string;
        romName?: string | null;
        romData?: Uint8Array | null;
    } = {}) {
        // in case a remote ROM loading operation has been
        // requested then loads it from the remote origin
        if (loadRom) {
            ({ name: romName, data: romData } =
                await Chip8Emulator.fetchRom(romPath));
        } else if (romName === null || romData === null) {
            [romName, romData] = [this.romName, this.romData];
        }

        // in case either the ROM's name or data is not available
        // throws an error as the boot process is not possible
        if (!romName || !romData) {
            throw new Error("Unable to load initial ROM");
        }

        // selects the proper engine for execution
        // and builds a new instance of it
        switch (engine) {
            case "neo":
                this.chip8 = new Chip8Neo();
                break;

            case "classic":
                this.chip8 = new Chip8Classic();
                break;

            default:
                if (!this.chip8) {
                    throw new Error("No engine requested");
                }
                break;
        }

        // resets the CHIP-8 engine to restore it into
        // a valid state ready to be used
        this.chip8.reset_hard_ws();
        this.chip8.load_rom_ws(romData);

        // updates the name of the currently selected engine
        // to the one that has been provided (logic change)
        if (engine) this._engine = engine;

        // updates the complete set of global information that
        // is going to be displayed
        this.setRom(romName, romData);

        // in case the restore (state) flag is set
        // then resumes the machine execution
        if (restore) this.resume();

        // triggers the booted event indicating that the
        // emulator has finished the loading process
        this.trigger("booted");
    }

    setRom(name: string, data: Uint8Array) {
        this.romName = name;
        this.romData = data;
        this.romSize = data.length;
    }

    get name(): string {
        return Info.name() ?? info.name;
    }

    get device(): Entry {
        return {
            text: Info.system(),
            url: "https://en.wikipedia.org/wiki/CHIP-8"
        };
    }

    get icon(): string | undefined {
        return require("../res/globe.png");
    }

    get version(): Entry | undefined {
        return {
            text: Info.version() ?? info.version,
            url: "https://github.com/joamag/chip-ahoyto/blob/master/CHANGELOG.md"
        };
    }

    get repository(): Entry {
        return {
            text: "GitHub",
            url: "https://github.com/joamag/chip-ahoyto"
        };
    }

    get features(): Feature[] {
        return [
            Feature.Themes,
            Feature.Palettes,
            Feature.Benchmark,
            Feature.Keyboard,
            Feature.KeyboardChip8,
            Feature.SaveState
        ];
    }

    get engines(): string[] {
        return ["neo", "classic"];
    }

    get engine(): string {
        return this._engine ?? "neo";
    }

    get romExts(): string[] {
        return ["ch8"];
    }

    get pixelFormat(): PixelFormat {
        return PixelFormat.RGB;
    }

    get dimensions(): Size {
        return {
            width: DISPLAY_WIDTH,
            height: DISPLAY_HEIGHT,
            scale: DISPLAY_SCALE
        };
    }

    get imageBuffer(): Uint8Array {
        const bufferMapped: number[] = [];
        const palette = PALETTES[this.paletteIndex];
        this.chip8?.vram_ws().forEach((value) => {
            if (value) {
                bufferMapped.push(...palette.colors[0]);
            } else {
                bufferMapped.push(...palette.colors[1]);
            }
        });
        return new Uint8Array(bufferMapped);
    }

    get romInfo(): RomInfo {
        return {
            name: this.romName ?? undefined,
            data: this.romData ?? undefined,
            size: this.romSize
        };
    }

    get frequency(): number {
        return this.logicFrequency;
    }

    set frequency(value: number) {
        value = Math.max(value, 0);
        this.logicFrequency = value;
        this.trigger("frequency", value);
    }

    get frequencySpecs(): FrequencySpecs {
        return {
            unit: Frequency.Hz,
            delta: 60,
            places: 0
        };
    }

    get framerate(): number {
        return this.fps;
    }

    get registers(): Record<string, string | number> {
        const registers = this.chip8?.registers_ws();
        if (!registers) return {};
        return {
            pc: registers.pc,
            i: registers.i,
            sp: registers.sp,
            dt: registers.dt,
            st: registers.st
        };
    }

    get palette(): string | undefined {
        const paletteObj = PALETTES[this.paletteIndex];
        return paletteObj.name;
    }

    set palette(value: string | undefined) {
        if (value === undefined) return;
        const paletteObj = PALETTES_MAP[value];
        this.paletteIndex = PALETTES.indexOf(paletteObj);
    }

    toggleRunning() {
        if (this.paused) {
            this.resume();
        } else {
            this.pause();
        }
    }

    pause(): void {
        this.paused = true;
    }

    resume() {
        this.paused = false;
        this.nextTickTime = EmulatorBase.now();
    }

    reset() {
        this.boot({ engine: null });
    }

    keyPress(key: string): void {
        const keyCode = KEYS[key];
        if (!keyCode) return;
        this.chip8?.key_press_ws(keyCode);
    }

    keyLift(key: string): void {
        const keyCode = KEYS[key];
        if (!keyCode) return;
        this.chip8?.key_lift_ws(keyCode);
    }

    serializeState(): Uint8Array {
        if (!this.chip8) throw new Error("Unable to serialize state");
        return this.chip8.get_state_ws();
    }

    unserializeState(data: Uint8Array) {
        if (!this.chip8) throw new Error("Unable to unserialize state");
        this.chip8.set_state_ws(data);
    }

    buildState(index: number): SaveState {
        return {
            index: index,
            timestamp: 0,
            agent: "CHIP-Ahoyto",
            model: "CHIP-8"
        };
    }

    changePalette(): string {
        this.paletteIndex += 1;
        this.paletteIndex %= PALETTES.length;
        return PALETTES[this.paletteIndex].name;
    }

    benchmark(count = 200000000): BenchmarkResult {
        let cycles = 0;
        this.pause();
        try {
            const initial = EmulatorBase.now();
            for (let i = 0; i < count; i++) {
                this.chip8?.clock_ws();
                cycles += 1;
            }
            const delta = (EmulatorBase.now() - initial) / 1000;
            const frequency_mhz = cycles / delta / 1000 / 1000;
            return {
                delta: delta,
                count: count,
                cycles: cycles,
                frequency_mhz: frequency_mhz
            };
        } finally {
            this.resume();
        }
    }

    private static async fetchRom(
        romPath: string
    ): Promise<{ name: string; data: Uint8Array }> {
        // extracts the name of the ROM from the provided
        // path by splitting its structure
        const romPathS = romPath.split(/\//g);
        let romName = romPathS[romPathS.length - 1].split("?")[0];
        const romNameS = romName.split(/\./g);
        romName = `${romNameS[0]}.${romNameS[romNameS.length - 1]}`;

        // loads the ROM data and converts it into the
        // target byte array buffer (to be used by WASM)
        const response = await fetch(romPath);
        const blob = await response.blob();
        const arrayBuffer = await blob.arrayBuffer();
        const romData = new Uint8Array(arrayBuffer);

        // returns both the name of the ROM and the data
        // contents as a byte array
        return {
            name: romName,
            data: romData
        };
    }
}

const beep = async () => {
    sound.muted = false;
    await sound.play();
};
