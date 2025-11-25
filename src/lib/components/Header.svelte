<!-- src/lib/components/Header.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
    import LanguageSelector from './LanguageSelector.svelte';
  import ThemeSelector from './ThemeSelector.svelte';
  import * as m from '$lib/paraglide/messages';
</script>

<header class="bg-base-200 p-4 flex items-center justify-between">
  <!-- Left -->
  <div class="flex items-center gap-4">
    <!-- <ThemeSelector /> -->

    <ThemeSelector>
        {#snippet icon()}
            <svg class="h-7 w-7 text-primary" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 2l7 5v10l-7 5-7-5V7l7-5z" />
            </svg>
        {/snippet}
    </ThemeSelector>

    <!-- <LanguageSelector /> -->

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
    {#each [{id: 'load', label: m.header_tab_load()}, {id: 'lexer', label: m.header_tab_lexer()}, {id: 'parser', label: m.header_tab_parser()}] as tab}
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

</header>