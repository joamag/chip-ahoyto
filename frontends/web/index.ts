import { startApp } from "emukit";

import { Chip8Emulator } from "./ts";

const BACKGROUNDS = [
    "264653",
    "1b1a17",
    "023047",
    "bc6c25",
    "283618",
    "2a9d8f",
    "3a5a40"
];

(async () => {
    // parses the current location URL as retrieves
    // some of the "relevant" GET parameters for logic
    const params = new URLSearchParams(window.location.search);
    const romUrl = params.get("rom_url") ?? params.get("url") ?? undefined;
    const verbose = ["1", "true", "True"].includes(params.get("verbose") ?? "");

    // creates the emulator structure and initializes the
    // React app with both the parameters and the emulator
    const emulator = new Chip8Emulator({ debug: verbose });
    await emulator.init();
    startApp("app", {
        emulator: emulator,
        backgrounds: BACKGROUNDS
    });
    await emulator.start({ romUrl: romUrl });
})();
