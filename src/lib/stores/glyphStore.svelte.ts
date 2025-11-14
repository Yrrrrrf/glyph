// src/lib/stores/glyphStore.svelte.ts
import { analyze_assembly } from "$lib/wasm";

export type TabState = "load" | "lexer" | "parser";
export type AnalysisState = "idle" | "loading" | "lexer_ready" | "error";

export interface HighlightInfo {
  line: number;
  element: string;
}

export interface WasmToken {
  element: string;
  token_type: string;
  line?: number;
}

// The Store - Arrow functions preserve `this` binding
class GlyphStore {
  sourceCode = $state<string>("");
  currentFile = $state<string | null>(null);
  analysisResult = $state<WasmToken[] | null>(null);

  activeTab = $state<TabState>("load");
  analysisState = $state<AnalysisState>("idle");
  error = $state<string | null>(null);
  highlightInfo = $state<HighlightInfo | null>(null);

  get HAS_FILE(): boolean {
    const result = this.currentFile !== null && this.sourceCode.length > 0;
    console.log("🔍 HAS_FILE:", result, "| file:", this.currentFile);
    return result;
  }

  get IS_ANALYZED(): boolean {
    return this.analysisState === "lexer_ready" && this.analysisResult !== null;
  }

  get TOKEN_COUNT(): number {
    return this.analysisResult?.length ?? 0;
  }

  // Arrow function = `this` is permanently bound
  runAnalysis = async (): Promise<void> => {
    console.log("⚙️ runAnalysis called");
    if (!this.HAS_FILE) {
      console.warn("⚠️ No file to analyze");
      return;
    }

    this.analysisState = "loading";
    this.error = null;

    try {
      const tokens = analyze_assembly(this.sourceCode);
      console.log("✅ Analysis done, tokens:", tokens.length);
      this.analysisResult = tokens;
      this.analysisState = "lexer_ready";
    } catch (error) {
      this.analysisState = "error";
      this.error = error instanceof Error ? error.message : "Analysis failed";
      console.error("❌ Analysis error:", this.error);
    }
  };

  // Arrow function = `this` is permanently bound
  loadFile = async (content: string, filename: string): Promise<void> => {
    console.log("📂 loadFile called:", filename);
    this.error = null;

    // Validate
    if (!filename.toLowerCase().endsWith(".asm")) {
      this.error = `Invalid file: "${filename}" is not a .asm file`;
      console.error("❌ Validation failed:", this.error);
      return;
    }

    // Set data
    this.sourceCode = content;
    this.currentFile = filename;
    console.log("✅ File set, switching tab and analyzing...");

    // Switch tab and run analysis
    this.activeTab = "lexer";
    await this.runAnalysis(); // `this` is guaranteed to work
    console.log("✅ loadFile complete");
  };

  setActiveTab = (tab: TabState): void => {
    console.log("🖱️ Tab changed to:", tab);
    this.activeTab = tab;
  };

  setHighlight = (info: HighlightInfo | null): void => {
    this.highlightInfo = info;
  };

  clearFile = (): void => {
    console.log("🗑️ Clearing file...");
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
