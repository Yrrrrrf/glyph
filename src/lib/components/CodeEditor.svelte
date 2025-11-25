<script lang="ts">
  import { glyphStore, getTokenTextClass } from '$lib/stores/glyphStore.svelte';
  import type { WasmToken } from '$lib/types/tokenTypes.svelte';
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';

  let { 
    code = $bindable(), 
    // We import these from store, but they are passed as props usually
    highlightInfo = glyphStore.highlightInfo, 
    selectedLine = glyphStore.selectedLine
  } = $props<{
    code: string;
    highlightInfo?: HighlightInfo | null;
    selectedLine?: number | null;
  }>();

  let textAreaRef = $state<HTMLTextAreaElement>();
  let preRef = $state<HTMLPreElement>();
  let backdropRef = $state<HTMLDivElement>();

  // Scroll Sync: Now syncing 3 layers (Backdrop, Pre, Textarea)
  function handleScroll() {
    if (textAreaRef && preRef && backdropRef) {
      const top = textAreaRef.scrollTop;
      const left = textAreaRef.scrollLeft;
      preRef.scrollTop = top;
      preRef.scrollLeft = left;
      backdropRef.scrollTop = top;
      // Backdrop usually doesn't need horizontal scroll for background lines, 
      // but good to keep in sync if we add content there.
    }
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    glyphStore.updateCode(target.value);
  }

  // --- HTML GENERATOR ---
  function generateHighlightedHTML(source: string, tokens: WasmToken[] | null, highlight: HighlightInfo | null): string {
    // Fallback if no tokens (just plain text, but handle comments)
    if (!tokens || tokens.length === 0) return processComments(escapeHtml(source));

    let html = "";
    let cursor = 0;
    const sortedTokens = [...tokens].sort((a, b) => a.start - b.start);

    for (const token of sortedTokens) {
        // 1. GAP Processing (Whitespace & Comments)
        if (token.start > cursor) {
            const gap = source.slice(cursor, token.start);
            html += processComments(escapeHtml(gap));
        }

        // 2. Token Processing
        const tokenText = source.slice(token.start, token.end);
        const colorClass = getTokenTextClass(token.category);
        
        // CHECK HOVER HIGHLIGHT
        // If current token matches the hover span
        const isHovered = highlight?.start === token.start; 
        
        // Construct Span
        let classes = colorClass;
        if (isHovered) {
            // "badge badge-outline" style, but applied to text
            classes += " outline outline-1 outline-primary rounded-[2px] bg-primary/10"; 
        }

        html += `<span class="${classes}">${escapeHtml(tokenText)}</span>`;
        cursor = token.end;
    }

    // 3. Trailing text
    if (cursor < source.length) {
        const gap = source.slice(cursor);
        html += processComments(escapeHtml(gap));
    }

    if (source.endsWith('\n')) html += '<br>'; 
    return html;
  }

  // Helper to dim anything after a semicolon
  function processComments(text: string): string {
    if (!text.includes(';')) return text;
    // Split by the first semicolon found
    const parts = text.split(';');
    // The first part is normal (whitespace mostly)
    let res = parts[0];
    // Reconstruct the rest as comment
    // We add the missing ';' back inside the span
    const commentBody = parts.slice(1).join(';');
    res += `<span class="text-base-content/40 italic font-medium">;${commentBody}</span>`;
    return res;
  }

  function escapeHtml(text: string): string {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  // Derived state
  let highlightedCode = $derived(generateHighlightedHTML(code, glyphStore.lexerResult, glyphStore.highlightInfo));
  let lines = $derived(code.split('\n'));
</script>

<div class="relative w-full h-full border border-base-300 rounded-lg overflow-hidden bg-base-100 flex shadow-inner">
    
    <!-- GUTTER (Line Numbers) -->
    <div class="flex-none w-12 bg-base-200/50 border-r border-base-300 text-right text-base-content/40 select-none font-mono text-xs pt-4 pb-4 z-10">
        {#each lines as _, i}
            <div 
                class="px-2 leading-6 h-6 transition-colors duration-150"
                class:text-primary={glyphStore.selectedLine === i + 1 || glyphStore.highlightInfo?.line === i + 1}
                class:font-bold={glyphStore.selectedLine === i + 1}
            >
                {i + 1}
            </div>
        {/each}
    </div>

    <!-- EDITOR CONTAINER -->
    <div class="relative flex-grow h-full overflow-hidden bg-base-100">
        
        <!-- LAYER 0: LINE HIGHLIGHTS (Background) -->
        <div bind:this={backdropRef} class="absolute inset-0 pt-4 pb-4 w-full pointer-events-none overflow-hidden">
            {#each lines as _, i}
                <div 
                    class="w-full h-6 transition-colors duration-75"
                    class:bg-warning={glyphStore.highlightInfo?.line === i + 1}
                    class:bg-opacity-10={glyphStore.highlightInfo?.line === i + 1}
                    class:bg-base-200={glyphStore.selectedLine === i + 1} 
                ></div>
            {/each}
        </div>

        <!-- LAYER 1: SYNTAX HIGHLIGHTS (Pre) -->
        <pre 
            bind:this={preRef}
            class="absolute inset-0 m-0 p-4 font-mono text-sm leading-6 whitespace-pre overflow-hidden pointer-events-none custom-scrollbar z-10"
            aria-hidden="true"
        >{@html highlightedCode}</pre>

        <!-- LAYER 2: INPUT (Textarea) -->
        <textarea
            bind:this={textAreaRef}
            value={code}
            oninput={handleInput}
            onscroll={handleScroll}
            class="absolute inset-0 w-full h-full m-0 p-4 font-mono text-sm leading-6 whitespace-pre bg-transparent text-transparent caret-primary resize-none focus:outline-none custom-scrollbar z-20"
            spellcheck="false"
            autocomplete="off"
        ></textarea>
    </div>
</div>