// src/lib/types/tokenTypes.svelte.ts
export interface WasmToken {
  element: string;
  category: string; // "instruction", "register", "constant", etc.
  detail: string; // "instruction-mov", "register-ax", "constant-decimal"
  line: number;
}

// For UI grouping
export type TokenCategory =
  | "instruction"
  | "directive"
  | "register"
  | "symbol"
  | "constant"
  | "punctuation"
  | "invalid";
