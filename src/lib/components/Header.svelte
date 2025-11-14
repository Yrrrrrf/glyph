<script lang="ts">
  import ThemeSelector from "./ThemeSelector.svelte";

  let { onAnalyze } = $props<{
    onAnalyze: (() => void) | (() => Promise<void>);
  }>();
</script>

<header class="bg-base-200 p-4 flex items-center justify-between">
  <div class="flex items-center gap-3">
    <!-- Glyph Icon -->
    <svg class="h-7 w-7 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" 
            d="M12 2l7 5v10l-7 5-7-5V7l7-5z" />
    </svg>
    <h1 class="text-xl font-bold">Glyph</h1>
    <ThemeSelector />
  </div>
  
  <button class="btn btn-primary btn-sm" onclick={onAnalyze}>
    <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
            d="M5 13l4 4L19 7" />
    </svg>
    Analyze
  </button>
</header>