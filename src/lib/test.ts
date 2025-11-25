// src/lib/test.ts

// this line specifies that we want to use Deno's standard library types
// ,ns extention
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
      console.error(`❌ Error reading '${filename}': ${error.message}`);
    }
    Deno.exit(1);
  }
}

function printLexerOutput(
  tokens: WasmToken[] | null,
  sourceCode: string,
): void {
  console.log("=== LEXER OUTPUT ===");

  if (!tokens || tokens.length === 0) {
    console.log("(No tokens produced)\n");
    return;
  }

  // Header with new columns
  console.log(
    "Line".padEnd(5) + " | " +
      "Span".padEnd(9) + " | " + // Start-End
      "Category".padEnd(20) + " | " +
      "Raw Text".padEnd(15) + " | " +
      "Detail",
  );
  console.log("-".repeat(100));

  for (const token of tokens) {
    const lineStr = String(token.line).padEnd(5);
    // Create span string: "0-5"
    const spanStr = `${token.start}-${token.end}`.padEnd(9);
    const catStr = token.category.padEnd(20);

    // Slice the original text to prove we have the correct indices
    const actualText = sourceCode.slice(token.start, token.end).padEnd(15);

    // Color logic
    let output =
      `${lineStr} | ${spanStr} | ${catStr} | ${actualText} | ${token.detail}`;

    if (
      token.category.includes("Error") || token.category.includes("invalid")
    ) {
      console.log(`\x1b[31m${output}\x1b[0m`);
    } else {
      console.log(output);
    }
  }
  console.log();
}

function printErrors(errors: string[]): void {
  if (errors.length === 0) return;

  console.log("=== COMPILER ERRORS ===");
  for (const err of errors) {
    console.log(`❌ ${err}`);
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

  // CAST the result to our known interface
  const rawResult = analyze_assembly(code) as unknown as JsCompilerResult;

  // 1. Print Lexer Output (if tokens exist)
  // Pass 'code' here so we can verify the slicing
  printLexerOutput(rawResult.tokens, code);

  // 2. Print Errors (Lexer, Parser, or Validator errors)
  printErrors(rawResult.errors);

  // 3. Status
  if (rawResult.success) {
    console.log("\n✅ Analysis Successful");
    if (rawResult.program) {
      console.log(
        `🌳 AST Generated with ${rawResult.program.length} statements.`,
      );
    }
  } else {
    console.log("\n⚠️ Analysis Completed with Errors");
    Deno.exit(1);
  }
}

if (import.meta.main) {
  main().catch((error) => {
    console.error("\n❌ Fatal error:", error.message);
    Deno.exit(1);
  });
}
