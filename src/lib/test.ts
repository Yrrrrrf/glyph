import { analyze_assembly, initWasm } from "./wasm";

const TEST_CODE = `MOV AX, 10h
ADD AX, 1`;

async function runTest() {
  await initWasm(); // Wait for WASM
  console.log("=== NEW LEXER OUTPUT (TS/WASM) ===");

  const tokens: any[] = analyze_assembly(TEST_CODE);

  tokens.forEach((t) => {
    console.log(`${t.token_type.padEnd(12)} | ${t.element}`);
  });

  console.log("\nTotal tokens:", tokens.length);
}

runTest();
