// src/lib/types/tokenTypes.svelte.ts

export interface WasmToken {
  element: string;
  category: string;
  detail: string;
  line: number;
}

export type TokenCategory =
  | "instruction"
  | "directive"
  | "register"
  | "symbol"
  | "constant"
  | "punctuation"
  | "invalid";

// --- NUEVO: Estructuras para la Fase 2 (Análisis Semántico) ---

export interface SymbolRecord {
  name: string;
  type_: "Variable" | "Label" | "Constant"; // Matches Rust "type_" field
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
