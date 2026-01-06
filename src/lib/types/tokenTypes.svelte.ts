// src/lib/types/tokenTypes.svelte.ts

export interface WasmToken {
  element: string; // The raw slice from source (e.g. "MOV", "0afH")
  category: string; // The high-level category (e.g. "Instruction", "Register")
  detail: string; // The specific type description (e.g. "Data Transfer", "Binary")
  line: number;
  start: number;
  end: number;
}

// Optional: You can refine this union type if you want strict checking in TS components
export type TokenCategory =
  | "Instruction"
  | "Directive"
  | "Register"
  | "Constant"
  | "Symbol"
  | "Punctuation"
  | "Error";

// (Keep the Phase 2 structures if you are using them in ParserView)
export interface SymbolRecord {
  name: string;
  type_: string;
  data_type: string;
  value: number;
  segment: string;
}

export interface LineAnalysis {
  line_number: number;
  is_correct: boolean;
  error_message: string | null;
  instruction: string;
  address: string | null;
  machine_code: string | null;
}

export interface AnalysisResult {
  symbol_table: SymbolRecord[];
  lines: LineAnalysis[];
}
