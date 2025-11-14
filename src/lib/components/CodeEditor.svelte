<script lang="ts">
  type HighlightInfo = { line: number; element: string } | null;

  let { code, highlight } = $props<{
    code: string;
    highlight: HighlightInfo;
  }>();

  export interface CodeEditorProps {
    code: string;
    highlight: HighlightInfo | null;
  }
  let lines = $derived(code.split('\n'));

  // Split line into parts, highlighting matches
  function getLineParts(line: string, element: string | null): Array<{ text: string; isMatch: boolean }> {
    if (!element || !line.includes(element)) {
      return [{ text: line, isMatch: false }];
    }

    // Escape regex special characters
    const escapedElement = element.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(`(${escapedElement})`, 'g');
    const parts = line.split(regex);
    
    return parts.map(part => ({
      text: part,
      isMatch: part === element
    }));
  }
</script>

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full">
  <div class="font-mono text-sm">
    {#each lines as line, i (i)} <!-- Use index as key for stability -->
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
              {#if part.isMatch}
                <span class="badge badge-primary badge-sm font-mono px-1 py-0 mx-0.5 align-middle">
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