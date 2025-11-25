// src/lib/test.ts
/// <reference lib="deno.ns" />

import { analyze_assembly, initWasm } from "./wasm.ts";
import type { JsCompilerResult } from "./types/analysisTypes.ts";
import type { WasmToken } from "./types/tokenTypes.svelte.ts";

function getFilename(): string {
  if (Deno.args.length === 0) {
    console.log("ℹ️  No filename provided, using built-in test code\n");
    return "test.asm";
  }
  return Deno.args[0];
}

function readFile(filename: string): string {
  try {
    const content = Deno.readTextFileSync(filename);
    console.log(`📄 Loaded ${content.length} characters from ${filename}\n`);
    return content;
  } catch (error) {
    if (error instanceof Error) {
      console.error(`Error reading '${filename}': ${error.message}`);
    }
    Deno.exit(1);
  }
}

function printLexerOutput(
  tokens: WasmToken[] | null,
): void {
  console.log("=== LEXER OUTPUT ===");

  // Matches Rust: "{:<4} | {:<48} | {:<24}"
  console.log(
    "Line".padEnd(4) + " | " +
      "Category".padEnd(48) + " | " +
      "Value (Source Slice)",
  );
  console.log("-".repeat(90));

  if (!tokens || tokens.length === 0) {
    console.log("(No tokens produced)\n");
    return;
  }

  for (const token of tokens) {
    const line = String(token.line).padEnd(4);
    // In JS/Wasm result: token.element is the Slice, token.detail is the Description
    const val = token.element.padEnd(24);

    // Check for Error category
    if (token.category === "Error" || token.category.includes("Invalid")) {
      // Rust: println!("{line:<4} | \x1b[31m{description:<48}\x1b[0m | {value_str:<24}");
      const desc = token.detail.padEnd(48);
      console.log(`${line} | \x1b[31m${desc}\x1b[0m | ${val}`);
    } else {
      // Rust: println!("{line:<4} | {category:<16}{description:<32} | {value_str:<24}");
      const cat = token.category.padEnd(16);
      const desc = token.detail.padEnd(32);
      console.log(`${line} | ${cat}${desc} | ${val}`);
    }
  }
  console.log();
}

function printErrors(errors: string[]): void {
  if (errors.length === 0) return;

  console.log("=== COMPILER ERRORS ===");
  for (const err of errors) {
    console.log(`${err}`);
  }
  console.log();
}

async function main(): Promise<void> {
  console.log("=== Glyph Test Runner ===\n");

  await initWasm();
  console.log("✅ WebAssembly module initialized\n");

  const filename = getFilename();
  const code = Deno.args.length > 0
    ? readFile(filename)
    : `MOV AX, 10h\nADD AX, 1`;

  console.log(`=== Processing: ${filename} ===\n`);

  // Analyze
  const rawResult = analyze_assembly(code) as unknown as JsCompilerResult;

  // 1. Print Lexer Output
  // Note: We don't need to pass 'code' anymore for slicing,
  // because the Rust side now does the slicing for us in token.element!
  printLexerOutput(rawResult.tokens);

  // 2. Print Errors
  printErrors(rawResult.errors);

  // 3. Status
  if (rawResult.success) {
    console.log("\nAnalysis Successful");
    if (rawResult.program) {
      console.log(
        `🌳 AST Generated with ${rawResult.program.length} statements.`,
      );
    }
  } else {
    console.log("\nAnalysis Completed with Errors");
    Deno.exit(1);
  }
}

main().catch((error) => {
  console.error("\natal error:", error.message);
  Deno.exit(1);
});
