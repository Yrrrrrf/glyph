// src/lib/wasm.ts
import init, { analyze_assembly, analyze_full_program } from "./pkg/glyph.js"; // <--- Add analyze_full_program

export { analyze_assembly, analyze_full_program }; // <--- Export it

let wasmReadyPromise: Promise<void> | null = null;

export function initWasm(): Promise<void> {
  if (wasmReadyPromise === null) {
    wasmReadyPromise = init()
      .then(() => console.log("✅ Glyph WASM module initialized."))
      .catch((err) => {
        console.error("❌ Error initializing WASM:", err);
        wasmReadyPromise = null;
        throw err;
      });
  }
  return wasmReadyPromise;
}
