// src/lib/stores/glyphStore.svelte.ts
import { analyze_assembly } from "$lib/wasm";
import type { WasmToken } from "$lib/types/tokenTypes.svelte";

export type TabState = "load" | "lexer" | "parser";
export type AnalysisState = "idle" | "loading" | "lexer_ready" | "error";

export interface HighlightInfo {
  line: number;
  element: string;
  detail: string;
}

export function getTokenBadgeClasses(type: string): string {
  // const baseClasses = "badge badge-sm font-mono px-2 py-0.5";
  // return `${baseClasses} ${(() => {
  return `${"badge badge-sm font-mono px-2 py-0.5"} ${
    (() => {
      switch (type) {
        case "instruction":
          return "badge-info text-info-content";
        case "directive":
          return "badge-accent text-accent-content";
        case "register":
          return "badge-success text-success-content";
        case "constant":
        case "string":
          return "badge-warning text-warning-content";
        case "punctuation":
          return "badge-outline border-base-400 text-base-content";
        case "invalid":
          return "badge-error text-error-content";
        case "symbol":
        default:
          return "badge-ghost bg-base-300";
      }
    })()
  }`;
}

// The Store - Arrow functions preserve `this` binding
class GlyphStore {
  // Add this to the GlyphStore class
  selectedLine = $state<number | null>(null);

  // Add this method
  setSelectedLine = (line: number | null): void => {
    this.selectedLine = line;
    // Auto-clear hover when selecting to avoid visual conflict
    if (line !== null) {
      this.highlightInfo = null;
    }
  };

  sourceCode = $state<string>("");
  currentFile = $state<string | null>(null);
  analysisResult = $state<WasmToken[] | null>(null);

  activeTab = $state<TabState>("load");
  analysisState = $state<AnalysisState>("idle");
  error = $state<string | null>(null);
  highlightInfo = $state<HighlightInfo | null>(null);

  get HAS_FILE(): boolean {
    const result = this.currentFile !== null && this.sourceCode.length > 0;
    console.log("File ->:", result, "| file:", this.currentFile);
    return result;
  }

  get IS_ANALYZED(): boolean {
    return this.analysisState === "lexer_ready" && this.analysisResult !== null;
  }

  get TOKEN_COUNT(): number {
    return this.analysisResult?.length ?? 0;
  }

  async runAnalysis(): Promise<void> {
    if (!this.HAS_FILE) return;

    this.analysisState = "loading";
    try {
      const tokens = analyze_assembly(this.sourceCode) as WasmToken[];
      this.analysisResult = tokens;
      this.analysisState = "lexer_ready";
    } catch (error) {
      this.analysisState = "error";
      this.error = error instanceof Error ? error.message : "Analysis failed";
    }
  }

  // Arrow function = `this` is permanently bound
  loadFile = async (content: string, filename: string): Promise<void> => {
    console.log("loadFile called:", filename);
    this.error = null;

    // Validate
    if (!filename.toLowerCase().endsWith(".asm")) {
      this.error = `Invalid file: "${filename}" is not a .asm file`;
      console.error("Validation failed:", this.error);
      return;
    }

    // Set data
    this.sourceCode = content;
    this.currentFile = filename;
    console.log("File set, switching tab and analyzing...");

    // Switch tab and run analysis
    this.activeTab = "lexer";
    await this.runAnalysis(); // `this` is guaranteed to work
    console.log("loadFile complete");
  };

  setActiveTab = (tab: TabState): void => {
    console.log("Tab changed to:", tab);
    this.activeTab = tab;
  };

  setHighlight = (info: HighlightInfo | null): void => {
    this.highlightInfo = info;
  };

  clearFile = (): void => {
    console.log("Clearing file...");
    this.sourceCode = "";
    this.currentFile = null;
    this.error = null;
    this.analysisResult = null;
    this.analysisState = "idle";
    this.highlightInfo = null;
    this.activeTab = "load";
  };
}

export const glyphStore = new GlyphStore();
