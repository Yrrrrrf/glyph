<script lang="ts">
  import { glyphStore, getTokenTextClass } from '$lib/stores/glyphStore.svelte';
  import type { WasmToken } from '$lib/types/tokenTypes.svelte';
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';

  let { 
    code = $bindable(), 
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
  let gutterRef = $state<HTMLDivElement>();

  // Constants based on Tailwind classes (leading-6 = 24px, p-4 = 16px)
  const LINE_HEIGHT = 24; 

  // --- AUTO SCROLL EFFECT ---
  $effect(() => {
    if (selectedLine && textAreaRef) {
        scrollToLine(selectedLine);
    }
  });

  function scrollToLine(line: number) {
    if (!textAreaRef) return;

    const currentScroll = textAreaRef.scrollTop;
    const clientHeight = textAreaRef.clientHeight;
    
    // Line 1 is at offset 0
    const lineOffset = (line - 1) * LINE_HEIGHT;
    
    // Buffer logic
    const isAbove = lineOffset < currentScroll;
    const isBelow = lineOffset > (currentScroll + clientHeight - LINE_HEIGHT * 2);

    if (isAbove || isBelow) {
        const targetScroll = lineOffset - (clientHeight / 2) + (LINE_HEIGHT / 2);
        textAreaRef.scrollTo({
            top: Math.max(0, targetScroll),
            behavior: 'smooth'
        });
    }
  }

  // --- SCROLL SYNC ---
  function handleScroll() {
    if (textAreaRef && preRef && backdropRef && gutterRef) {
      const top = textAreaRef.scrollTop;
      const left = textAreaRef.scrollLeft;
      preRef.scrollTop = top;
      preRef.scrollLeft = left;
      backdropRef.scrollTop = top;
      gutterRef.scrollTop = top; 
    }
  }

  // --- CURSOR SYNC ---
  function handleCursorMove() {
    if (!textAreaRef) return;
    const cursorIndex = textAreaRef.selectionStart;
    const textUpToCursor = textAreaRef.value.substring(0, cursorIndex);
    const lineNumber = textUpToCursor.split('\n').length;

    if (glyphStore.selectedLine !== lineNumber) {
        glyphStore.setSelectedLine(lineNumber);
    }
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;
    glyphStore.updateCode(target.value);
    handleCursorMove();
  }

  // --- HTML GENERATOR ---
  function generateHighlightedHTML(source: string, tokens: WasmToken[] | null, highlight: HighlightInfo | null): string {
    if (!tokens || tokens.length === 0) return processComments(escapeHtml(source));

    let html = "";
    let cursor = 0;
    const sortedTokens = [...tokens].sort((a, b) => a.start - b.start);

    for (const token of sortedTokens) {
        if (token.start > cursor) {
            html += processComments(escapeHtml(source.slice(cursor, token.start)));
        }
        const tokenText = source.slice(token.start, token.end);
        const colorClass = getTokenTextClass(token.category);
        const isHovered = highlight?.start === token.start; 
        
        let classes = colorClass;
        if (isHovered) {
            classes += " outline outline-1 outline-primary rounded-[2px] bg-primary/10"; 
        }

        html += `<span class="${classes}">${escapeHtml(tokenText)}</span>`;
        cursor = token.end;
    }

    if (cursor < source.length) {
        html += processComments(escapeHtml(source.slice(cursor)));
    }
    if (source.endsWith('\n')) html += '<br>'; 
    return html;
  }

  function processComments(text: string): string {
    if (!text.includes(';')) return text;
    const parts = text.split(';');
    let res = parts[0];
    const commentBody = parts.slice(1).join(';');
    res += `<span class="text-base-content/40 italic font-medium">;${commentBody}</span>`;
    return res;
  }

  function escapeHtml(text: string): string {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  let highlightedCode = $derived(generateHighlightedHTML(code, glyphStore.lexerResult, glyphStore.highlightInfo));
  let lines = $derived(code.split('\n'));
</script>

<div class="relative w-full h-full border border-base-300 rounded-lg overflow-hidden bg-base-100 flex shadow-inner">
    
    <!-- GUTTER -->
    <div 
        bind:this={gutterRef}
        class="flex-none w-12 bg-base-200/50 border-r border-base-300 text-right text-base-content/40 select-none font-mono text-xs pt-4 pb-4 z-10 overflow-hidden"
    >
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
        
        <!-- LAYER 0: LINE HIGHLIGHTS & SELECTION BAR -->
        <div bind:this={backdropRef} class="absolute inset-0 pt-4 pb-4 w-full pointer-events-none overflow-hidden">
            {#each lines as _, i}
                {@const isHovered = glyphStore.highlightInfo?.line === i + 1}
                {@const isSelected = glyphStore.selectedLine === i + 1}

                <div 
                    class="w-full h-6 transition-colors duration-75 border-l-4"

                    class:border-transparent={!isSelected && !isHovered}
                    class:border-primary={isSelected || isHovered}

                    class:bg-primary={isHovered}
                    class:bg-opacity-10={isHovered}
                    
                    class:bg-base-200={isSelected && !isHovered} 
                ></div>
            {/each}
        </div>

        <!-- LAYER 1: SYNTAX HIGHLIGHTS -->
        <pre 
            bind:this={preRef}
            class="absolute inset-0 m-0 p-4 font-mono text-sm leading-6 whitespace-pre overflow-hidden pointer-events-none custom-scrollbar z-10"
            aria-hidden="true"
        >{@html highlightedCode}</pre>

        <!-- LAYER 2: INPUT -->
        <textarea
            bind:this={textAreaRef}
            value={code}
            oninput={handleInput}
            onscroll={handleScroll}
            onclick={handleCursorMove}
            onkeyup={handleCursorMove}
            class="absolute inset-0 w-full h-full m-0 p-4 font-mono text-sm leading-6 whitespace-pre bg-transparent text-transparent caret-primary resize-none focus:outline-none custom-scrollbar z-20"
            spellcheck="false"
            autocomplete="off"
        ></textarea>
    </div>
</div>