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
*/
  clock_ws(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chip8neo_free: (a: number) => void;
  readonly chip8neo_new: () => number;
  readonly chip8neo_load_rom_ws: (a: number, b: number, c: number) => void;
  readonly chip8neo_reset_ws: (a: number) => void;
  readonly chip8neo_reset_hard_ws: (a: number) => void;
  readonly chip8neo_clock_ws: (a: number) => void;
  readonly __wbg_chip8classic_free: (a: number) => void;
  readonly chip8classic_new: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
