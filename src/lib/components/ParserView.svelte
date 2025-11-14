<!-- src/lib/components/ParserView.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
  
  let { ast, error } = $props<{ ast: any | null; error: Error | null }>();
</script>

<div class="h-full p-8">
  {#if error}
    <div class="alert alert-error mb-4">
      <span>❌ Parser Error: {error.message}</span>
    </div>
  {/if}
  
  {#if ast}
    <div class="bg-base-200 rounded-lg p-4 h-full overflow-auto">
      <pre class="font-mono text-sm">{JSON.stringify(ast, null, 2)}</pre>
    </div>
  {:else}
    <div class="text-center flex flex-col items-center justify-center h-full">
      <div class="text-6xl mb-4">🌳</div>
      <h3 class="text-xl font-semibold mb-2">AST Visualization</h3>
      <p class="text-base-content/70 max-w-md">
        Abstract Syntax Tree will appear here once parser is integrated.
      </p>
    </div>
  {/if}
</div>