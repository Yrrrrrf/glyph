import { initWasm } from "./wasm";

// Initializes the Glyph library.
export async function init(): Promise<void> {
  await initWasm();
  // todo: Load other possible initial data here!
  // * :D
}

// * Re-export types for easier consumption in +page.svelte or other layouts
export type * from "./types/analysisTypes";
export type * from "./types/tokenTypes.svelte";
