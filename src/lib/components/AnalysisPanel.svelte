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

  let tableContainerRef = $state<HTMLDivElement>();
  let rowRefs: Record<number, HTMLTableRowElement> = {};

  // EFFECT: Scroll to the selected line when it changes in the Store (from Editor)
  $effect(() => {
    if (selectedLine !== null && tokens.length > 0 && tableContainerRef) {
        // Find the first token that matches the line
        const targetIndex = tokens.findIndex((t: any) => t.line === selectedLine);
        
        if (targetIndex !== -1) {
            const row = rowRefs[targetIndex];
            if (row) {
                row.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
        }
    }
  });

  function isRowHighlighted(token: WasmToken): boolean {
    if (!highlightedInfo) return false;
    // Specific token hover
    if (highlightedInfo.start !== undefined) {
        return token.start === highlightedInfo.start;
    }
    // Broad line hover
    return token.line === highlightedInfo.line;
  }
</script>

<div 
  bind:this={tableContainerRef}
  class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full custom-scrollbar"
>
  <table class="table table-xs table-pin-rows">
    <thead>
      <tr>
        <th class="w-12">Ln</th>
        <!-- <th class="w-20">Span</th> -->
        <th>Value</th>
        <th>Category / Detail</th> 
      </tr>
    </thead>
    <tbody>
      {#each tokens as token, idx (idx)}
        <tr
          bind:this={rowRefs[idx]}
          onmouseenter={() => onTokenHover({ 
            line: token.line, 
            element: token.element,
            detail: token.detail,
            start: token.start,
            end: token.end
          })}
          onmouseleave={() => onTokenHover(null)}
          onclick={() => onTokenSelect(token.line)}
          class="hover:bg-base-200 transition-colors duration-75 cursor-pointer"
          class:bg-warning={isRowHighlighted(token)}
          class:bg-opacity-20={isRowHighlighted(token)}
          class:bg-base-200={selectedLine === token.line} 
          class:border-l-4={selectedLine === token.line}
          class:border-primary={selectedLine === token.line}
        >
          <td class="font-mono text-base-content/50">{token.line}</td>
          <!-- <td class="font-mono text-xs text-base-content/40">{token.start}-{token.end}</td> -->
          
          <td class="font-mono font-bold text-primary whitespace-pre">{token.element}</td>
          
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