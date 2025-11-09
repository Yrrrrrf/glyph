import { initWasm } from "./wasm";

// Initializes the Glyph library.
export async function init(): Promise<void> {
  await initWasm();
  // todo: Load other possible initial data here!
  // * :D
}
