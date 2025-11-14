<!-- src/lib/components/AnalysisPanel.svelte -->
<script lang="ts">
  import type { WasmToken } from '$lib/stores/glyphStore.svelte';
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';

  let { 
    tokens, 
    onTokenHover, 
    highlightedInfo 
  } = $props<{
    tokens: WasmToken[];
    onTokenHover: (info: HighlightInfo | null) => void;
    highlightedInfo: HighlightInfo | null;
  }>();

  function isTokenHighlighted(token: WasmToken): boolean {
    return highlightedInfo !== null && 
           token.line === highlightedInfo.line && 
           token.element === highlightedInfo.element;
  }
</script>

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full">
  <table class="table table-xs">
    <thead>
      <tr>
        <th class="w-16">Line</th>
        <th>Element</th>
        <th class="w-24">Type</th>
      </tr>
    </thead>
    <tbody>
      {#each tokens as token, idx (idx)}
        <tr
          onmouseenter={() => onTokenHover({ 
            line: token.line ?? 1, 
            element: token.element 
          })}
          onmouseleave={() => onTokenHover(null)}
          class="hover:bg-base-200 transition-colors duration-150 cursor-pointer"
          class:bg-base-300={isTokenHighlighted(token)}
        >
          <td class="font-mono text-sm">{token.line ?? 1}</td>
          <td class="font-mono text-sm">{token.element}</td>
          <td class="text-sm capitalize">{token.token_type}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>