import type { WasmToken } from "./tokenTypes.svelte.ts";

// The structure returned by src/lib.rs -> JsCompilerResult
export interface JsCompilerResult {
  success: boolean;
  tokens: WasmToken[] | null;
  errors: string[];
  program: any | null; // The AST (defined loosely here as it's a complex nested object)
}

// Keep these for the Frontend components (ParserView),
// though currently Rust isn't returning them calculated yet.
export interface SymbolRecord {
  name: string;
  type_: "Variable" | "Label" | "Constant";
  data_type: "Byte" | "Word" | "Dword" | "None";
  value: number;
  segment: string;
}

export interface LineAnalysis {
  line_number: number;
  is_correct: boolean;
  error_message: string | null;
  instruction: string;
}

export interface AnalysisResult {
  symbol_table: SymbolRecord[];
  lines: LineAnalysis[];
}
