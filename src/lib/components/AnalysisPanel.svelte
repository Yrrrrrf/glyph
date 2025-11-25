<script lang="ts">
  import type { WasmToken } from '$lib/types/tokenTypes.svelte';
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';
  import { getTokenBadgeClasses } from '$lib/stores/glyphStore.svelte';

  let { 
    tokens, 
    onTokenHover, 
    onTokenSelect, 
    highlightedInfo,
    selectedLine 
  } = $props<{
    tokens: WasmToken[];
    onTokenHover: (info: HighlightInfo | null) => void;
    onTokenSelect: (line: number) => void;
    highlightedInfo: HighlightInfo | null;
    selectedLine: number | null;
  }>();

  // Highlight logic for the Table Row
  function isRowHighlighted(token: WasmToken): boolean {
    if (!highlightedInfo) return false;
    // Check if spans match (precise) or just line (broad)
    if (highlightedInfo.start !== undefined) {
        return token.start === highlightedInfo.start;
    }
    return token.line === highlightedInfo.line;
  }

  function getTokenType(detail: string): string {
    return detail.split('-')[0];
  }

  function formatDetail(detail: string): string {
    // Regex to extract inner part of Enum: "Constant(NumberHex(...))" -> "NumberHex"
    // Or just clean up the Rust debug output slightly
    return detail.replace(/([A-Z][a-z]+)\((.*)\)/, '$1: $2');
  }
</script>

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full custom-scrollbar">
  <table class="table table-xs table-pin-rows">
    <thead>
      <tr>
        <th class="w-12">Ln</th>
        <th class="w-20">Span</th>
        <th>Value</th>
        <th>Category / Detail</th> 
      </tr>
    </thead>
    <tbody>
      {#each tokens as token, idx (idx)}
        <tr
          onmouseenter={() => onTokenHover({ 
            line: token.line ?? 1, 
            element: token.element,
            detail: token.detail,
            start: token.start, // <--- PASS INDICES
            end: token.end
          })}
          onmouseleave={() => onTokenHover(null)}
          onclick={() => onTokenSelect(token.line)}
          class="hover:bg-base-200 transition-colors duration-75 cursor-pointer"
          class:bg-warning={isRowHighlighted(token)}
          class:bg-opacity-20={isRowHighlighted(token)}
        >
          <td class="font-mono text-base-content/50">{token.line}</td>
          <td class="font-mono text-xs text-base-content/40">{token.start}-{token.end}</td>
          
          <!-- Element (Raw text) -->
          <td class="font-mono font-bold text-primary">{token.element}</td>
          
          <td>
            <span class={getTokenBadgeClasses(token.category)}>
              {token.category}
            </span>
            <span class="text-[10px] opacity-50 ml-2 font-mono block md:inline">
                {token.detail}
            </span>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>