<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
  import FileInput from '$lib/components/FileInput.svelte';
  import Header from '$lib/components/Header.svelte';
  import CodeEditor from '$lib/components/CodeEditor.svelte';
  import AnalysisPanel from '$lib/components/AnalysisPanel.svelte';
  import ParserView from '$lib/components/ParserView.svelte';
</script>

<div class="flex flex-col h-screen overflow-hidden">
  <Header />
  
  {#if glyphStore.error}
    <div class="bg-error text-error-content p-2 text-center text-xs font-bold">
      {glyphStore.error}
    </div>
  {/if}
  
  <main class="flex-1 overflow-hidden p-4">
    {#if glyphStore.activeTab === 'load'}
      <div class="flex flex-col items-center justify-center h-full gap-6">
        <div class="text-6xl animate-bounce">📁</div>
        <h2 class="text-2xl font-bold">Load Assembly File</h2>
        <p class="text-base-content/60">Or switch to the "Lexer" tab to start typing from scratch.</p>
        <FileInput onFileLoaded={glyphStore.loadFile} />
      </div>
      
    {:else if glyphStore.activeTab === 'lexer'}
      <div class="flex h-full gap-4">
        <!-- Editor Column -->
        <div class="flex-1 flex flex-col min-w-0">
          <div class="flex justify-between items-center mb-2">
            <h3 class="text-lg font-semibold flex items-center gap-2">
                <span>📝</span> Code Editor
            </h3>
            <span class="text-xs text-base-content/50">
                {glyphStore.analysisState === 'loading' ? 'Analyzing...' : 'Ready'}
            </span>
          </div>
          
          <div class="flex-1 min-h-0">
             <CodeEditor 
                bind:code={glyphStore.sourceCode} 
                highlightInfo={glyphStore.highlightInfo}
                selectedLine={glyphStore.selectedLine}
             />
          </div>
        </div>

        <!-- Analysis Column -->
        <div class="flex-1 flex flex-col min-w-0">
          <h3 class="text-lg font-semibold mb-2 flex items-center gap-2">
            <span>🔍</span> Token Stream
          </h3>
          <div class="flex-1 min-h-0">
            <AnalysisPanel
                tokens={glyphStore.lexerResult ?? []} 
                onTokenHover={glyphStore.setHighlight}
                onTokenSelect={glyphStore.setSelectedLine}
                highlightedInfo={glyphStore.highlightInfo}
                selectedLine={glyphStore.selectedLine}
            />
          </div>
        </div>
      </div>
      
    {:else if glyphStore.activeTab === 'parser'}
      <ParserView />
    {/if}
  </main>
</div>