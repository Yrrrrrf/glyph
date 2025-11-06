// Import the default export 'init' along with your function
import init, { greet } from './pkg/glyph.js';

async function run() {
  // 1. Await the initialization of the WASM module.
  //    This loads the .wasm file and links everything together.
  await init();

  // 2. Now it is safe to call your functions.
  greet("World from Deno!");
}

run();