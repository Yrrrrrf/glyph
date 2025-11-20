// src/lib/stores/glyphStore.svelte.ts
import { analyze_assembly, analyze_full_program } from "$lib/wasm";
import type { WasmToken } from "$lib/types/tokenTypes.svelte";
import type { AnalysisResult } from "$lib/types/analysisTypes";

export type TabState = "load" | "lexer" | "parser";
export type AnalysisState = "idle" | "loading" | "ready" | "error";

export interface HighlightInfo {
  line: number;
  element: string;
  detail: string;
}

// Helper for badge colors
export function getTokenBadgeClasses(type: string): string {
  if (type.startsWith("instruction")) return "badge-info text-info-content";
  if (type.startsWith("register")) return "badge-success text-success-content";
  if (type.startsWith("constant") || type.startsWith("string")) {
    return "badge-warning text-warning-content";
  }
  if (type.startsWith("punctuation")) return "badge-outline";
  if (type.startsWith("directive")) return "badge-accent text-accent-content";
  if (type === "invalid") return "badge-error text-error-content";
  return "badge-ghost";
}

class GlyphStore {
  // State
  sourceCode = $state<string>("");
  currentFile = $state<string | null>(null);

  // --- DATA STORAGE ---
  // RENAME THIS: analysisResult -> lexerResult
  lexerResult = $state<WasmToken[] | null>(null);
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
    // Update reference here too
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
    this.lexerResult = null; // Clear lexer
    this.parserResult = null; // Clear parser
    this.analysisState = "idle";
    this.activeTab = "load";
  };

  runAnalysis = async () => {
    if (!this.HAS_FILE) return;
    this.analysisState = "loading";
    this.error = null;

    try {
      // PHASE 1: LEXER
      const tokens = analyze_assembly(this.sourceCode) as WasmToken[];
      this.lexerResult = tokens; // <--- SAVE TO lexerResult

      // PHASE 2: PARSER
      const semanticResult = analyze_full_program(
        this.sourceCode,
      ) as AnalysisResult;
      this.parserResult = semanticResult;

      this.analysisState = "ready";
    } catch (err) {
      this.analysisState = "error";
      this.error = err instanceof Error ? err.message : "Analysis failed";
    }
  };
}

export const glyphStore = new GlyphStore();
