<!-- src/routes/+page.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
  import FileInput from '$lib/components/FileInput.svelte';
  import Header from '$lib/components/Header.svelte';
  import CodeEditor from '$lib/components/CodeEditor.svelte';
  import AnalysisPanel from '$lib/components/AnalysisPanel.svelte';

  // DEBUG: Watch all store changes
  $effect(() => {
    console.log('🎭 PAGE - Store changed:', {
      file: glyphStore.currentFile,
      tab: glyphStore.activeTab,
      tokens: glyphStore.TOKEN_COUNT,
      error: glyphStore.error,
      hasFile: glyphStore.HAS_FILE
    });
  });
</script>

<div class="flex flex-col h-screen">
  <Header />
  
  <!-- ERROR DISPLAY -->
  {#if glyphStore.error}
    <div class="bg-error text-error-content p-3 text-center text-sm">
      {glyphStore.error}
    </div>
  {/if}
  
  <main class="flex-1 overflow-hidden p-4">
    {#if glyphStore.activeTab === 'load'}
      <div class="flex flex-col items-center justify-center h-full gap-6">
        <div class="text-6xl">📁</div>
        <h2 class="text-2xl font-bold">Load Assembly File</h2>
        <FileInput onFileLoaded={glyphStore.loadFile} />
      </div>
      
    {:else if glyphStore.activeTab === 'lexer'}
      <div class="flex h-full gap-4">
        <div class="flex-1">
          <h3 class="text-lg font-semibold mb-2">Code Editor</h3>
          <CodeEditor code={glyphStore.sourceCode} highlight={glyphStore.highlightInfo} />
        </div>
        <div class="flex-1">
          <h3 class="text-lg font-semibold mb-2">Token Analysis</h3>
          <AnalysisPanel
            tokens={glyphStore.analysisResult ?? []}
            onTokenHover={glyphStore.setHighlight}
            highlightedInfo={glyphStore.highlightInfo}
          />
        </div>
      </div>
      
    {:else if glyphStore.activeTab === 'parser'}
      <div class="flex items-center justify-center h-full">
        <div class="text-center">
          <div class="text-6xl mb-4">🚧</div>
          <h3 class="text-xl font-semibold">Parser Under Construction</h3>
        </div>
      </div>
    {/if}
  </main>
</div>