<script lang="ts">
  import type { WasmToken } from '$lib/types/tokenTypes.svelte';
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';
  import { getTokenTextClass } from '$lib/stores/glyphStore.svelte';
  import * as m from '$lib/paraglide/messages';

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
  let isHoveringTable = $state(false);

  // --- I18N MAPPING HELPERS ---
  const norm = (s: string) => s.toLowerCase().replace(/ /g, '_');

  function translateCategory(cat: string): string {
    const key = `token_cat_${norm(cat)}`;
    const msgFunc = (m as any)[key];
    return typeof msgFunc === 'function' ? msgFunc() : cat;
  }

  function translateDetail(det: string): string {
    if (det.startsWith("Error")) {
        return (m as any).token_cat_error ? (m as any).token_cat_error() : "Error";
    }
    const key = `token_det_${norm(det)}`;
    const msgFunc = (m as any)[key];
    return typeof msgFunc === 'function' ? msgFunc() : det;
  }

  // --- STYLING HELPERS ---
  function getBadgeClass(cat: string): string {
    const c = cat.toLowerCase();
    // Matching Code Editor Colors roughly
    if (c === 'instruction') return 'badge-info text-info-content';       // Blue
    if (c === 'register') return 'badge-success text-success-content';    // Green
    if (c === 'directive') return 'badge-secondary text-secondary-content'; // Purple/Pink
    if (c === 'constant') return 'badge-warning text-warning-content';    // Orange/Yellow
    if (c === 'symbol') return 'badge-neutral text-neutral-content';      // Grey/Dark
    if (c === 'punctuation') return 'badge-ghost opacity-60';             // Transparent
    if (c === 'error') return 'badge-error text-error-content';           // Red
    return 'badge-ghost';
  }

  // --- SCROLL LOGIC ---
  $effect(() => {
    if (!isHoveringTable && selectedLine !== null && tokens.length > 0 && tableContainerRef) {
        const targetIndex = tokens.findIndex((t: any) => t.line === selectedLine);
        if (targetIndex !== -1 && rowRefs[targetIndex]) {
            rowRefs[targetIndex].scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    }
  });

  function isRowHighlighted(token: WasmToken): boolean {
    if (!highlightedInfo) return false;
    if (highlightedInfo.start !== undefined) return token.start === highlightedInfo.start;
    return token.line === highlightedInfo.line;
  }
</script>

<div 
  bind:this={tableContainerRef}
  class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full custom-scrollbar"
  role="group"
  onmouseenter={() => isHoveringTable = true}
  onmouseleave={() => isHoveringTable = false}
>

  <table class="table table-xs table-pin-rows">
    <thead>
      <tr class="bg-base-200/50 backdrop-blur-sm z-10">
        <th class="w-12 text-base-content/50 font-normal">{m.analysis_panel_ln()}</th>
        <th class="text-base-content/50 font-normal">{m.analysis_panel_value()}</th>
        <th class="text-base-content/50 font-normal">{m.analysis_panel_category_detail()}</th> 
      </tr>
    </thead>
    <tbody>
      {#each tokens as token, idx (idx)}
        {@const isHighlighted = isRowHighlighted(token)}
        {@const isSelected = selectedLine === token.line}
        
        <tr
          bind:this={rowRefs[idx]}
          onmouseenter={() => {
            onTokenHover({ 
              line: token.line, 
              element: token.element,
              detail: token.detail,
              start: token.start,
              end: token.end
            });
            onTokenSelect(token.line);
          }}
          onmouseleave={() => onTokenHover(null)}
          onclick={() => onTokenSelect(token.line)}
          
          class="transition-all duration-75 cursor-pointer border-l-4"
          
          class:border-transparent={!isSelected && !isHighlighted}
          class:bg-base-200={isSelected && !isHighlighted}
          class:border-base-content={isSelected && !isHighlighted}
          
          class:bg-primary={isHighlighted}
          class:bg-opacity-10={isHighlighted}
          class:border-primary={isHighlighted}
        >
          <!-- Line Number -->
          <td class="font-mono text-base-content/40 select-none w-12">
            {token.line}
          </td>
          
          <!-- Value (Colored Text) -->
          <td class="font-mono text-sm max-w-[120px] break-all">
             <span class={getTokenTextClass(token.category)}>
                {token.element}
             </span>
          </td>
          
          <!-- Category / Detail -->
          <td>
            <div class="flex flex-col gap-1 items-start justify-center h-full py-1">
                <!-- Category Badge -->
                <span class="badge badge-sm font-bold border-none h-auto py-0.5 min-h-[1.25rem] {getBadgeClass(token.category)}">
                    {translateCategory(token.category)}
                </span>

                <!-- Detail Text (Only if different) -->
                {#if token.detail && token.detail !== token.category}
                    <span class="text-[10px] uppercase tracking-wider opacity-60 font-medium leading-none ml-1">
                        {translateDetail(token.detail)}
                    </span>
                {/if}
            </div>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>