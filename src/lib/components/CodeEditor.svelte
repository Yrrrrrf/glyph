<script lang="ts">
  import type { HighlightInfo } from '$lib/stores/glyphStore.svelte';
  import { getTokenBadgeClasses } from '$lib/stores/glyphStore.svelte';


  let { 
    code, 
    highlight,
    scrollToLine = $bindable(null)
  } = $props<{
    code: string;
    highlight: (HighlightInfo & { detail?: string; }) | null;
    scrollToLine?: number | null;
  }>();

  let editorRef = $state<HTMLDivElement>();
  
  $effect(() => {
    if (scrollToLine && editorRef) {
      const lineElements = editorRef.querySelectorAll('[data-line]');
      const targetLine = lineElements[scrollToLine - 1];
      if (targetLine) {
        targetLine.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
      // Clear after scrolling
      setTimeout(() => scrollToLine = null, 100);
    }
  });

  let lines = $derived(code.split('\n'));

  function getLineParts(line: string, element: string | null): Array<{ text: string; isMatch: boolean }> {
    if (!element || !line.includes(element)) {
      return [{ text: line, isMatch: false }];
    }
    const escapedElement = element.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(`(${escapedElement})`, 'g');
    const parts = line.split(regex);
    return parts.map(part => ({
      text: part,
      isMatch: part === element
    }));
  }
</script>

<div bind:this={editorRef} class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full custom-scrollbar">

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full">
  <div class="font-mono text-sm">
    {#each lines as line, i (i)}
      <div 
        class="flex hover:bg-base-200 transition-colors duration-150" 
        class:bg-base-300={highlight?.line === i + 1}
      >
        <div class="w-12 flex-shrink-0 px-3 py-1 text-right text-gray-500 select-none border-r border-base-300">
          {i + 1}
        </div>
        <div class="px-3 py-1 flex-grow whitespace-pre">
          {#if highlight?.line === i + 1 && highlight?.element}
            {#each getLineParts(line, highlight.element) as part, j (j)}
              {#if part.isMatch && highlight.detail}
                <span class={getTokenBadgeClasses(highlight.detail)}>
                  {part.text}
                </span>
              {:else}
                {part.text}
              {/if}
            {/each}
          {:else}
            {line}
          {/if}
        </div>
      </div>
    {/each}
  </div>
</div>
</div>