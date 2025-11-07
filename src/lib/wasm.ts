// src/lib/wasm.ts

// Import the default init function and all named exports from the wasm-pack generated code.
import init, { greet } from "./pkg/glyph.js";

// --- Direct WASM Function Exports ---
export { greet };

// --- WASM Initialization Singleton ---
let wasmReadyPromise: Promise<void> | null = null;

/**
 * Initializes the WebAssembly module for the entire application.
 * This should be called once in the root layout to ensure all WASM functions
 * are ready before any component attempts to call them.
 *
 * @returns {Promise<void>} A promise that resolves when the module is ready.
 */
export function initWasm(): Promise<void> {
  if (wasmReadyPromise === null) {
    wasmReadyPromise = init()
      .then(() => {
        console.log("✅ Glyph WASM module initialized successfully.");
      })
      .catch((error) => {
        console.error("❌ Error initializing Glyph WASM module:", error);
        // Reset the promise on failure to allow a potential retry.
        wasmReadyPromise = null;
        // Re-throw the error so the calling component knows initialization failed.
        throw error;
      });
  }
  // Return the existing promise to any subsequent callers.
  return wasmReadyPromise;
}

/**
 * Represents the initialization state of the WebAssembly module.
 * 'loading': The module is currently being fetched and compiled.
 * 'ready': The module is initialized and its functions can be called.
 * 'error': An error occurred during initialization.
 */
// let wasmState: 'loading' | 'ready' | 'error' = $state('loading');
// let errorMessage = $state('');
