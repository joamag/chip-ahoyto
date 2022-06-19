/* tslint:disable */
/* eslint-disable */
/**
*/
export class Chip8Classic {
  free(): void;
/**
*/
  constructor();
}
/**
*/
export class Chip8Neo {
  free(): void;
/**
*/
  constructor();
/**
* @param {Uint8Array} rom
*/
  load_rom_ws(rom: Uint8Array): void;
/**
*/
  reset_ws(): void;
/**
*/
  reset_hard_ws(): void;
/**
* @returns {Uint8Array}
*/
  vram_ws(): Uint8Array;
/**
*/
  clock_ws(): void;
/**
*/
  clock_dt_ws(): void;
/**
*/
  clock_st_ws(): void;
/**
* @param {number} key
*/
  key_press_ws(key: number): void;
/**
* @param {number} key
*/
  key_lift_ws(key: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chip8neo_free: (a: number) => void;
  readonly chip8neo_new: () => number;
  readonly chip8neo_load_rom_ws: (a: number, b: number, c: number) => void;
  readonly chip8neo_reset_ws: (a: number) => void;
  readonly chip8neo_reset_hard_ws: (a: number) => void;
  readonly chip8neo_vram_ws: (a: number, b: number) => void;
  readonly chip8neo_clock_ws: (a: number) => void;
  readonly chip8neo_clock_dt_ws: (a: number) => void;
  readonly chip8neo_clock_st_ws: (a: number) => void;
  readonly chip8neo_key_press_ws: (a: number, b: number) => void;
  readonly chip8neo_key_lift_ws: (a: number, b: number) => void;
  readonly __wbg_chip8classic_free: (a: number) => void;
  readonly chip8classic_new: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
