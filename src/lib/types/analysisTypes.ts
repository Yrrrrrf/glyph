import type { WasmToken } from "./tokenTypes.svelte.ts";

export interface JsCompilerResult {
  success: boolean;
  tokens: WasmToken[] | null;
  errors: string[];
  // 'program' is the AST. Since it's a complex Rust enum structure,
  // 'any' is acceptable until you strictly type the AST on the frontend.
  program: any | null;
}

// Re-exporting these here for convenience if your components import from here
export type {
  AnalysisResult,
  LineAnalysis,
  SymbolRecord,
} from "./tokenTypes.svelte.ts";
