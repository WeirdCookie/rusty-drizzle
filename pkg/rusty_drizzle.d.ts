/* tslint:disable */
/* eslint-disable */

export class Rain {
    free(): void;
    [Symbol.dispose](): void;
    draw(ctx: CanvasRenderingContext2D, w: number, h: number): void;
    constructor(count: number, w: number, h: number);
    set_angle_deg(v: number): void;
    set_count(v: number, w: number, _h: number): void;
    set_size(v: number): void;
    set_speed(v: number): void;
    set_width(v: number): void;
    step(dt: number, w: number, h: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_rain_free: (a: number, b: number) => void;
    readonly rain_draw: (a: number, b: any, c: number, d: number) => void;
    readonly rain_new: (a: number, b: number, c: number) => number;
    readonly rain_set_angle_deg: (a: number, b: number) => void;
    readonly rain_set_count: (a: number, b: number, c: number, d: number) => void;
    readonly rain_set_size: (a: number, b: number) => void;
    readonly rain_set_speed: (a: number, b: number) => void;
    readonly rain_set_width: (a: number, b: number) => void;
    readonly rain_step: (a: number, b: number, c: number, d: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
