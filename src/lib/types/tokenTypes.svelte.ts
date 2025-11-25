// src/lib/types/tokenTypes.svelte.ts

export interface WasmToken {
  element: string;
  category: string;
  detail: string;
  line: number;
  // New Span Fields
  start: number;
  end: number;
}

export type TokenCategory =
  | "instruction"
  | "directive"
  | "register"
  | "symbol"
  | "constant"
  | "punctuation"
  | "invalid";

// (Keep the Phase 2 structures as they were)
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
