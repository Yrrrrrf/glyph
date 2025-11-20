<script lang="ts">
  import type { WasmToken } from '$lib/types/tokenTypes.svelte';
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';
  import { getTokenBadgeClasses } from '$lib/stores/glyphStore.svelte';

let { 
  tokens, 
  onTokenHover, 
  onTokenSelect, // NEW
  highlightedInfo,
  selectedLine // NEW
} = $props<{
  tokens: WasmToken[];
  onTokenHover: (info: HighlightInfo | null) => void;
  onTokenSelect: (line: number) => void; // NEW
  highlightedInfo: HighlightInfo | null;
  selectedLine: number | null; // NEW
}>();

// Add helper (or rename existing one)
function isLineSelected(token: WasmToken): boolean {
  return selectedLine === token.line;
}

  function isTokenHighlighted(token: WasmToken): boolean {
    return highlightedInfo !== null && 
           token.line === highlightedInfo.line && 
           token.element === highlightedInfo.element;
  }

  // Extract type prefix for coloring
  function getTokenType(detail: string): string {
    return detail.split('-')[0];
  }

  // Color based on type only, show full type-category text
  
  // Show full type-category in uppercase
  function formatDetail(detail: string): string {
    return detail.toUpperCase();
  }
</script>
<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full custom-scrollbar">

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full">
  <table class="table table-xs">
    <thead>
      <tr>
        <th class="w-16">Line</th>
        <th>Element</th>
        <th class="w-40">Type-Category</th> <!-- Wider column -->
      </tr>
    </thead>
    <tbody>
      {#each tokens as token, idx (idx)}
        <tr
          onmouseenter={() => onTokenHover({ 
            line: token.line ?? 1, 
            element: token.element,
            detail: token.detail // Pass full detail
          })}
          onmouseleave={() => onTokenHover(null)}
          class="hover:bg-base-200 transition-colors duration-150 cursor-pointer"
          class:bg-base-300={isTokenHighlighted(token)}
        >
          <td class="font-mono text-sm">{token.line ?? 1}</td>
          <td class="font-mono text-sm">{token.element}</td>
          <td>
            <span class={getTokenBadgeClasses(token.detail)}>
              {formatDetail(token.detail)}
            </span>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
</div>