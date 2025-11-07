import { initWasm } from "./wasm";

// Initializes the Glyph library.
export async function init(): Promise<void> {
  console.log("Welcome to Glyph");
  await initWasm();
}
