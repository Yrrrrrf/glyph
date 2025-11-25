import type {
  LineAnalysis,
  SymbolRecord,
  WasmToken,
} from "./tokenTypes.svelte.ts";

export interface JsCompilerResult {
  success: boolean;
  tokens: WasmToken[] | null;
  errors: string[];
  program: any | null;
  symbol_table: SymbolRecord[];
  line_analysis: LineAnalysis[];
}

export type {
  AnalysisResult,
  LineAnalysis,
  SymbolRecord,
} from "./tokenTypes.svelte.ts";
