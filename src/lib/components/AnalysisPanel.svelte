<script lang="ts">
  type Token = { line: number; element: string; type: string };
  type HighlightInfo = { line: number; element: string } | null;

  let { tokens, onTokenHover, highlightedInfo } = $props<{
    tokens: Token[];
    onTokenHover: (info: HighlightInfo) => void;
    highlightedInfo: HighlightInfo;
  }>();

  // Check if this token is the currently highlighted one
  function isTokenHighlighted(token: Token): boolean {
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
{#each tokens as token, idx (idx)} <!-- Use index for stable keys -->
  <tr
    onmouseenter={() => onTokenHover({ line: token.line, element: token.element })}
    onmouseleave={() => onTokenHover(null)}
    class="hover:bg-base-200 transition-colors duration-150 cursor-pointer"
    class:bg-base-300={highlightedInfo !== null && 
                       token.line === highlightedInfo.line && 
                       token.element === highlightedInfo.element}
  >
    <td class="font-mono text-sm">{token.line}</td>
    <td class="font-mono text-sm">{token.element}</td>
    <td class="text-sm capitalize">{token.type}</td>
  </tr>
{/each}
    </tbody>
  </table>
</div>