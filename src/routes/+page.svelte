<!-- src/routes/+page.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
  import FileInput from '$lib/components/FileInput.svelte';
  import Header from '$lib/components/Header.svelte';
  import CodeEditor from '$lib/components/CodeEditor.svelte';
  import AnalysisPanel from '$lib/components/AnalysisPanel.svelte';
  import ParserView from '$lib/components/ParserView.svelte';
</script>

<div class="flex flex-col h-screen">
  <Header />
  
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
          <!-- FIXED: Use lexerResult here -->
          <AnalysisPanel
            tokens={glyphStore.lexerResult ?? []} 
            onTokenHover={glyphStore.setHighlight}
            onTokenSelect={glyphStore.setSelectedLine}
            highlightedInfo={glyphStore.highlightInfo}
            selectedLine={glyphStore.selectedLine}
          />
        </div>
      </div>
      
    {:else if glyphStore.activeTab === 'parser'}
      <ParserView />
    {/if}
  </main>
</div>