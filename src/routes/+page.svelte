<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  import CodeEditor from '$lib/components/CodeEditor.svelte';
  import AnalysisPanel from '$lib/components/AnalysisPanel.svelte';
  import FileInput from '$lib/components/FileInput.svelte';
  import { analyze_assembly } from '$lib/wasm';

  let sourceCode = $state('');
  let analysisTokens = $state<any[]>([]);
  let highlightedInfo = $state<{ line: number; element: string } | null>(null);
  let currentFile = $state<string | null>(null);
  let activeTab = $state<'load' | 'lexer' | 'parser'>('load');

  function handleTokenHover(info: { line: number; element: string } | null) {
    highlightedInfo = info;
  }

  function handleFileLoaded(content: string, filename: string) {
    sourceCode = content;
    currentFile = filename;
    activeTab = 'lexer';
    runAnalysis();
  }

  async function runAnalysis() {
    try {
      const tokens = analyze_assembly(sourceCode);
      analysisTokens = tokens;
    } catch (err) {
      console.error('Analysis failed:', err);
      alert(`Analysis failed: ${err}`);
    }
  }

  function handleTabChange(tab: 'load' | 'lexer' | 'parser') {
    activeTab = tab;
  }
</script>


<div class="flex flex-col h-screen">
	<Header onAnalyze={runAnalysis} />
	<li>{currentFile}</li>
  
  <main class="flex-1 overflow-hidden p-4">
      <!-- Tabs -->
      <div class="tabs tabs-lift">
        <label class="tab">
          <input 
            type="radio" 
            name="main_tabs" 
            checked={activeTab === 'load'}
            onchange={() => handleTabChange('load')}
          />
          📁 Load
        </label>
        <label class="tab">
          <input 
            type="radio" 
            name="main_tabs" 
            checked={activeTab === 'lexer'}
            onchange={() => handleTabChange('lexer')}
          />
          🔍 Lexer
        </label>
        <label class="tab">
          <input 
            type="radio" 
            name="main_tabs" 
            checked={activeTab === 'parser'}
            onchange={() => handleTabChange('parser')}
          />
          🌳 Parser
        </label>
      </div>
      
      <!-- Tab Content -->
      <div class="flex-1 overflow-hidden bg-base-100 border border-base-300 rounded-b-lg rounded-tr-lg">
        {#if activeTab === 'load'}
          <div class="p-8">
            <FileInput onFileLoaded={handleFileLoaded} />
          </div>
        {:else if activeTab === 'lexer' && sourceCode}
          <div class="flex h-full gap-4 p-4">
            <div class="flex-1">
              <h3 class="text-lg font-semibold mb-2">Code Editor</h3>
              <CodeEditor code={sourceCode} highlight={highlightedInfo} />
            </div>
            <div class="flex-1">
              <h3 class="text-lg font-semibold mb-2">Token Analysis</h3>
              <AnalysisPanel
                tokens={analysisTokens}
                onTokenHover={handleTokenHover}
                highlightedInfo={highlightedInfo}
              />
            </div>
          </div>
        {:else if activeTab === 'parser'}
          <div class="p-8 text-center flex flex-col items-center justify-center h-full">
            <div class="text-6xl mb-4">🚧</div>
            <h3 class="text-xl font-semibold mb-2">Parser Under Construction</h3>
            <p class="text-base-content/70 max-w-md">
              Abstract Syntax Tree generation and semantic analysis will appear here once implemented.
            </p>
          </div>
        {/if}
      </div>
    <!-- {/if} -->
  </main>
</div>