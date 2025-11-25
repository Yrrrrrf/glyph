// src/lib/stores/glyphStore.svelte.ts
import { analyze_assembly, analyze_full_program } from "$lib/wasm";
import type { WasmToken } from "$lib/types/tokenTypes.svelte";
import type {
  AnalysisResult,
  JsCompilerResult,
} from "$lib/types/analysisTypes";

export type TabState = "load" | "lexer" | "parser";
export type AnalysisState = "idle" | "loading" | "ready" | "error";

export interface HighlightInfo {
  line: number;
  element: string; // Text content
  detail: string;
  start?: number;
  end?: number;
}

// UI Badge Colors (For Analysis Panel)
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
  if (t.includes("error") || t.includes("invalid")) {
    return "badge-error text-error-content";
  }
  return "badge-ghost";
}

// Syntax Highlighting Colors (For Editor Text)
export function getTokenTextClass(category: string): string {
  const c = category.toLowerCase();
  if (c.includes("instruction")) {
    return "text-blue-600 dark:text-blue-400 font-bold";
  }
  if (c.includes("register")) {
    return "text-green-600 dark:text-green-400 font-bold";
  }
  if (c.includes("constant")) return "text-orange-600 dark:text-orange-400";
  if (c.includes("string")) return "text-yellow-600 dark:text-yellow-400";
  if (c.includes("directive") || c.includes("pseudo")) {
    return "text-purple-600 dark:text-purple-400";
  }
  if (c.includes("symbol")) return "text-base-content";
  if (c.includes("error") || c.includes("invalid")) {
    return "text-red-500 underline decoration-wavy";
  }
  return "text-base-content";
}

class GlyphStore {
  // State
  sourceCode = $state<string>("");
  currentFile = $state<string | null>(null);

  // --- DATA STORAGE ---
  lexerResult = $state<WasmToken[] | null>(null);
  parserResult = $state<AnalysisResult | null>(null);

  // UI State
  activeTab = $state<TabState>("load");
  analysisState = $state<AnalysisState>("idle");
  error = $state<string | null>(null);
  highlightInfo = $state<HighlightInfo | null>(null);
  selectedLine = $state<number | null>(null);

  // Debounce Timer
  private debounceTimer: number | undefined = undefined;

  get HAS_FILE(): boolean {
    return this.sourceCode.length > 0; // Relaxed check (allow editing from scratch)
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

  // Called when file is loaded from disk
  loadFile = async (content: string, filename: string) => {
    this.error = null;
    if (!filename.toLowerCase().endsWith(".asm")) {
      this.error = "Invalid file type. Please upload .asm";
      return;
    }
    this.currentFile = filename;
    this.updateCode(content); // Use updateCode to trigger analysis
    this.activeTab = "lexer";
  };

  // Called when typing in the editor
  updateCode = (newCode: string) => {
    this.sourceCode = newCode;

    // Clear existing timer
    clearTimeout(this.debounceTimer);

    // Set new timer (300ms debounce)
    this.debounceTimer = setTimeout(() => {
      this.runAnalysis();
    }, 300) as unknown as number;
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
    if (!this.sourceCode.trim()) return;
    this.analysisState = "loading";
    // Don't clear error immediately so UI doesn't flash

    try {
      // Analyze
      const rawResult = analyze_full_program(
        this.sourceCode,
      ) as unknown as JsCompilerResult;

      // 1. Lexer
      if (rawResult.tokens) {
        this.lexerResult = rawResult.tokens;
      } else {
        this.lexerResult = [];
      }

      // 2. Parser (Placeholder for future AST integration)
      if (rawResult.program) {
        this.parserResult = {
          lines: [], // Fill this when Rust returns line analysis
          symbol_table: [],
        };
      }

      // 3. Errors (Only block if catastrophic, otherwise show tokens)
      if (!rawResult.success && rawResult.errors.length > 0) {
        // We might want to show errors in a specific panel instead of a global banner
        // For now, let's just log them or set a small warning
        // this.error = rawResult.errors[0];
      } else {
        this.error = null;
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
