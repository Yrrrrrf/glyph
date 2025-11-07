import { initWasm } from "./wasm";

export async function init(): Promise<void> {
  console.log("Welcome to Glyph");
  await initWasm();
}
