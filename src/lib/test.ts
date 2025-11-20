// src/lib/test.ts
import { analyze_assembly, initWasm } from "./wasm.ts";

interface WasmToken {
  element: string;
  category: string;
  detail: string;
  line: number;
}

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
    console.error(`❌ Error reading '${filename}': ${error.message}`);
    Deno.exit(1);
  }
}

// UPDATED: Now shows 4 columns to match Rust's debug output
function printLexerOutput(tokens: WasmToken[]): void {
  console.log("=== LEXER OUTPUT ===");

  if (tokens.length === 0) {
    console.log("(No tokens found)\n");
    return;
  }

  console.log(`Total tokens: ${tokens.length}\n`);

  // Match Rust's format: "Line {:>3} | {:>12} | {:?}"
  // Added detail column for full type information
  for (const token of tokens) {
    console.log(
      `Line ${String(token.line).padStart(3)} | ` +
        `${token.category.padStart(12)} | ` +
        `${token.element.padEnd(30)} | ` +
        `${token.detail}`,
    );
  }
}

function printParserOutput(tokens: WasmToken[]): void {
  console.log("\n=== PARSER OUTPUT ===");
  console.log("(Parser not yet integrated in TypeScript)");
  console.log(`Total tokens available: ${tokens.length}`);
}

function parseTokens(rawResult: any): WasmToken[] {
  console.log("🔍 Debugging WASM result structure...");
  console.log("   Type:", typeof rawResult);
  console.log("   Is Array?", Array.isArray(rawResult));

  if (!Array.isArray(rawResult)) {
    throw new Error(`Expected array from WASM, got ${typeof rawResult}`);
  }

  console.log(`   Array length: ${rawResult.length}`);

  if (rawResult.length > 0) {
    const first = rawResult[0];
    console.log("   First element:", first);
    console.log("   First element type:", typeof first);
    console.log(
      "   First element keys:",
      typeof first === "object" && first ? Object.keys(first) : "N/A",
    );
  }

  return rawResult.map((item, index) => {
    const element = item?.element !== undefined
      ? String(item.element)
      : `<?token_${index}?>`;
    const category = item?.category !== undefined
      ? String(item.category)
      : "unknown";
    const detail = item?.detail !== undefined ? String(item.detail) : "";
    const line = item?.line !== undefined ? Number(item.line) : 0;

    return { element, category, detail, line };
  });
}

async function main(): Promise<void> {
  console.log("=== Glyph Test Runner ===\n");

  await initWasm();
  console.log("✅ WebAssembly module initialized\n");

  const filename = getFilename();
  const code = Deno.args.length > 0 ? readFile(filename) : `MOV AX, 10h
ADD AX, 1`;

  console.log(`=== Processing: ${filename} ===\n`);

  const rawResult = analyze_assembly(code);
  const tokens = parseTokens(rawResult);

  printLexerOutput(tokens);
  printParserOutput(tokens);

  console.log("\n✅ Test complete!");
}

if (import.meta.main) {
  main().catch((error) => {
    console.error("\n❌ Fatal error:", error.message);
    Deno.exit(1);
  });
}
