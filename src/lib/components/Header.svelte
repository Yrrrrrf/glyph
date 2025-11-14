<!-- src/lib/components/Header.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
  import ThemeSelector from './ThemeSelector.svelte';

    // DEBUG: Watch store changes in the component (this is allowed!)
  // $effect(() => {
  //   console.log('🧩 HEADER - Store update:', {
  //     file: glyphStore.currentFile,
  //     tab: glyphStore.activeTab,
  //     tokens: glyphStore.TOKEN_COUNT
  //   });
  // });
</script>

<header class="bg-base-200 p-4 flex items-center justify-between">
  <!-- Left -->
  <div class="flex items-center gap-4">
    <svg class="h-7 w-7 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" 
            d="M12 2l7 5v10l-7 5-7-5V7l7-5z" />
    </svg>
    <h1 class="text-xl font-bold">Glyph</h1>
    
    {#if glyphStore.currentFile}
      <div class="hidden md:flex items-center gap-2 bg-base-300 px-3 py-1 rounded-lg">
        <span class="font-mono text-xs truncate max-w-xs">{glyphStore.currentFile}</span>
        <button class="btn btn-xs btn-square btn-ghost" onclick={glyphStore.clearFile}>
          ✕
        </button>
      </div>
    {/if}
  </div>

  <!-- Center: Tabs (No disabled logic) -->
  <div class="tabs tabs-boxed tabs-sm">
    {#each [{id: 'load', label: '📁 Load'}, {id: 'lexer', label: '🔍 Lexer'}, {id: 'parser', label: '🌳 Parser'}] as tab}
      <button 
        class="tab {glyphStore.activeTab === tab.id ? 'tab-active' : ''}"
        onclick={() => glyphStore.setActiveTab(tab.id as any)}
      >
        {tab.label}
        {#if tab.id === 'lexer' && glyphStore.TOKEN_COUNT > 0}
          ({glyphStore.TOKEN_COUNT})
        {/if}
      </button>
    {/each}
  </div>

  <!-- Right -->
  <div class="flex items-center gap-2">
    {#if glyphStore.HAS_FILE}
      <button 
        class="btn btn-primary btn-sm" 
        onclick={glyphStore.runAnalysis}
        disabled={glyphStore.analysisState === 'loading'}
      >
        {#if glyphStore.analysisState === 'loading'}
          <span class="loading loading-spinner loading-xs"></span>
        {:else}
          <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                  d="M5 13l4 4L19 7" />
          </svg>
        {/if}
        Analyze
      </button>
    {/if}
    
    <ThemeSelector />
  </div>
</header>