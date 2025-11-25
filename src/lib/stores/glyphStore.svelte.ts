// src/lib/stores/glyphStore.svelte.ts
import { analyze_full_program } from "$lib/wasm";
import type { WasmToken } from "$lib/types/tokenTypes.svelte";
import type {
  AnalysisResult,
  JsCompilerResult,
} from "$lib/types/analysisTypes";

export type TabState = "load" | "lexer" | "parser";
export type AnalysisState = "idle" | "loading" | "ready" | "error";

export interface HighlightInfo {
  line: number;
  element: string;
  detail: string;
}

// Helper for badge colors
export function getTokenBadgeClasses(type: string): string {
  if (!type) return "badge-ghost";
  const t = type.toLowerCase();
  if (t.startsWith("instruction")) return "badge-info text-info-content";
  if (t.startsWith("register")) return "badge-success text-success-content";
  if (t.startsWith("constant") || t.startsWith("string")) {
    return "badge-warning text-warning-content";
  }
  if (t.startsWith("punctuation")) return "badge-outline";
  if (t.startsWith("directive")) return "badge-accent text-accent-content";
  if (t === "invalid" || t.includes("error")) {
    return "badge-error text-error-content";
  }
  return "badge-ghost";
}

class GlyphStore {
  // State
  sourceCode = $state<string>("");
  currentFile = $state<string | null>(null);

  // --- DATA STORAGE ---
  lexerResult = $state<WasmToken[] | null>(null);

  // Note: ParserResult in the frontend expects SymbolTable/Lines.
  // Currently Rust returns AST. You will need to bridge this later.
  // For now, we store null or need to map the AST to this view.
  parserResult = $state<AnalysisResult | null>(null);

  // UI State
  activeTab = $state<TabState>("load");
  analysisState = $state<AnalysisState>("idle");
  error = $state<string | null>(null);
  highlightInfo = $state<HighlightInfo | null>(null);
  selectedLine = $state<number | null>(null);

  get HAS_FILE(): boolean {
    return this.currentFile !== null && this.sourceCode.length > 0;
  }

  get TOKEN_COUNT(): number {
    return this.lexerResult?.length ?? 0;
  }

  setSelectedLine = (line: number | null) => {
    this.selectedLine = line;
    if (line !== null) this.highlightInfo = null;
  };

  setHighlight = (info: HighlightInfo | null) => {
    this.highlightInfo = info;
  };

  setActiveTab = (tab: TabState) => {
    this.activeTab = tab;
  };

  loadFile = async (content: string, filename: string) => {
    this.error = null;
    if (!filename.toLowerCase().endsWith(".asm")) {
      this.error = "Invalid file type. Please upload .asm";
      return;
    }
    this.sourceCode = content;
    this.currentFile = filename;

    await this.runAnalysis();
    this.activeTab = "lexer";
  };

  clearFile = () => {
    this.sourceCode = "";
    this.currentFile = null;
    this.lexerResult = null;
    this.parserResult = null;
    this.analysisState = "idle";
    this.activeTab = "load";
  };

  runAnalysis = async () => {
    if (!this.HAS_FILE) return;
    this.analysisState = "loading";
    this.error = null;

    try {
      // Call Rust Wrapper
      // analyze_full_program now handles both lexing and parsing steps internally
      const rawResult = analyze_full_program(
        this.sourceCode,
      ) as unknown as JsCompilerResult;

      // 1. Handle Lexer Tokens
      if (rawResult.tokens) {
        this.lexerResult = rawResult.tokens;
      } else {
        this.lexerResult = [];
      }

      // 2. Handle Errors
      if (!rawResult.success && rawResult.errors.length > 0) {
        // Join the top errors for the UI banner
        this.error = rawResult.errors.slice(0, 2).join(". ");
      }

      // 3. Handle Parser Result
      // TODO: Your Rust backend currently returns `program` (AST), but the frontend
      // expects `AnalysisResult` (Symbol Table + Line Analysis).
      // For now, we leave this null or valid to prevent crashes,
      // waiting for Phase 2B implementation in Rust.
      if (rawResult.program) {
        // Placeholder to prevent UI crash if tab is clicked
        this.parserResult = {
          lines: [],
          symbol_table: [],
        };
      }

      this.analysisState = rawResult.success ? "ready" : "error";
    } catch (err) {
      this.analysisState = "error";
      this.error = err instanceof Error ? err.message : "Analysis failed";
      console.error(err);
    }
  };
}

export const glyphStore = new GlyphStore();
