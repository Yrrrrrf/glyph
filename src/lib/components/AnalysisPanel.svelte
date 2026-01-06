<!-- src/lib/components/AnalysisPanel.svelte -->
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

  // --- FILTER LOGIC ---
  // Create a derived view that excludes Punctuation tokens
  let visibleTokens = $derived(tokens.filter((t: any) => t.category !== 'Punctuation' && t.category !== 'Control'));

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
    if (c === 'instruction') return 'badge-info text-info-content';       
    if (c === 'register') return 'badge-success text-success-content';    
    if (c === 'directive') return 'badge-secondary text-secondary-content'; 
    if (c === 'constant') return 'badge-warning text-warning-content';    
    if (c === 'symbol') return 'badge-neutral text-neutral-content';      
    if (c === 'error') return 'badge-error text-error-content';           
    return 'badge-ghost';
  }

  // --- SCROLL LOGIC ---
  
  // Pagination State
  let currentPage = $state(1);
  const itemsPerPage = 50;

  let totalPages = $derived(Math.max(1, Math.ceil(visibleTokens.length / itemsPerPage)));
  
  let paginatedTokens = $derived(
    visibleTokens.slice((currentPage - 1) * itemsPerPage, currentPage * itemsPerPage)
  );

  function goToPage(page: number) {
    if (page >= 1 && page <= totalPages) {
        currentPage = page;
        // Reset scroll to top of table when changing page manually
        if (tableContainerRef) tableContainerRef.scrollTop = 0;
    }
  }

  $effect(() => {
    // Check if we need to switch pages to show the selected line
    if (!isHoveringTable && selectedLine !== null && visibleTokens.length > 0) {
        const targetIndex = visibleTokens.findIndex((t: any) => t.line === selectedLine);
        if (targetIndex !== -1) {
            const requiredPage = Math.floor(targetIndex / itemsPerPage) + 1;
            if (currentPage !== requiredPage) {
                currentPage = requiredPage;
            }
        }
    }
  });

  $effect(() => {
    // Scroll to the specific row if it's on the current page
    // We rely on Svelte's reactivity: when currentPage changes, the DOM updates, 
    // and then this effect (or a tick) allows us to access rowRefs.
    if (!isHoveringTable && selectedLine !== null && visibleTokens.length > 0) {
         const targetIndex = visibleTokens.findIndex((t: any) => t.line === selectedLine);
         if (targetIndex !== -1) {
             const pageOfTarget = Math.floor(targetIndex / itemsPerPage) + 1;
             if (pageOfTarget === currentPage) {
                 const localIndex = targetIndex % itemsPerPage;
                 if (rowRefs[localIndex]) {
                     rowRefs[localIndex].scrollIntoView({ behavior: 'smooth', block: 'center' });
                 }
             }
         }
    }
  });

  function isRowHighlighted(token: WasmToken): boolean {
    if (!highlightedInfo) return false;
    if (highlightedInfo.start !== undefined) return token.start === highlightedInfo.start;
    return token.line === highlightedInfo.line;
  }
</script>

<div class="flex flex-col h-full gap-2">
    <div 
    bind:this={tableContainerRef}
    class="bg-base-100 border border-base-300 rounded-lg overflow-auto flex-1 custom-scrollbar"
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
        <!-- Iterate over paginatedTokens -->
        {#each paginatedTokens as token, idx (idx)}
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
            
            <!-- Value -->
            <td class="font-mono text-sm max-w-[120px] break-all">
                <span class={getTokenTextClass(token.category)}>
                    {token.element}
                </span>
            </td>
            
            <!-- Category / Detail -->
            <td>
                <div class="flex flex-col gap-1 items-start justify-center h-full py-1">
                    <span class="badge badge-sm font-bold border-none h-auto py-0.5 min-h-[1.25rem] {getBadgeClass(token.category)}">
                        {translateCategory(token.category)}
                    </span>

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

    <!-- Pagination Controls -->
    {#if totalPages > 1}
    <div class="flex justify-between items-center px-2 py-1 bg-base-100 border border-base-300 rounded-lg shrink-0">
        <button 
            class="btn btn-xs btn-ghost" 
            disabled={currentPage === 1}
            onclick={() => goToPage(currentPage - 1)}
        >
            « Anterior
        </button>
        
        <span class="text-xs font-mono opacity-50">
            Página {currentPage} de {totalPages}
        </span>
        
        <button 
            class="btn btn-xs btn-ghost" 
            disabled={currentPage === totalPages}
            onclick={() => goToPage(currentPage + 1)}
        >
            Siguiente »
        </button>
    </div>
    {/if}
</div>