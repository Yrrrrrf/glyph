<!-- src/lib/components/ParserView.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';

  // Helper to format hex numbers
  const toHex = (num: number) => {
    return "0x" + num.toString(16).toUpperCase().padStart(4, '0');
  };
</script>

<div class="flex flex-col h-full gap-4">
  
  {#if !glyphStore.parserResult}
    <div class="flex flex-col items-center justify-center h-full opacity-50">
      <div class="text-4xl mb-2">⏳</div>
      <p>No analysis data available. Load a file to begin.</p>
    </div>
  {:else}
    
    <!-- TOP HALF: LINE ANALYSIS (Phase 2A) -->
    <div class="flex-1 flex flex-col min-h-0 border border-base-300 rounded-lg bg-base-100">
      <div class="p-3 bg-base-200 border-b border-base-300 flex justify-between items-center">
        <h3 class="font-bold text-sm flex items-center gap-2">
          <span>📋</span> Line Analysis
        </h3>
        <div class="badge badge-neutral text-xs">
            {glyphStore.parserResult.lines.filter(l => !l.is_correct).length} Errors
        </div>
      </div>

      <div class="overflow-auto custom-scrollbar flex-1">
        <table class="table table-xs table-pin-rows w-full">
          <thead>
            <tr>
              <th class="w-12">Ln</th>
              <th>Instruction / Source</th>
              <th class="w-24 text-center">Status</th>
              <th>Details</th>
            </tr>
          </thead>
          <tbody>
            {#each glyphStore.parserResult.lines as line (line.line_number)}
              <tr class="hover:bg-base-200 {line.is_correct ? '' : 'bg-error/5'}">
                <!-- Line Number -->
                <td class="font-mono text-base-content/60">{line.line_number}</td>
                
                <!-- Instruction Text -->
                <td class="font-mono font-medium">
                    {line.instruction}
                </td>

                <!-- Status Badge -->
                <td class="text-center">
                  {#if line.is_correct}
                    <span class="badge badge-xs badge-success gap-1">
                      OK
                    </span>
                  {:else}
                    <span class="badge badge-xs badge-error gap-1 font-bold">
                      ERR
                    </span>
                  {/if}
                </td>

                <!-- Error Message -->
                <td class="text-error text-xs font-semibold">
                  {line.error_message || ''}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- BOTTOM HALF: SYMBOL TABLE (Phase 2B) -->
    <div class="flex-1 flex flex-col min-h-0 border border-base-300 rounded-lg bg-base-100">
      <div class="p-3 bg-base-200 border-b border-base-300">
        <h3 class="font-bold text-sm flex items-center gap-2">
          <span>🔑</span> Symbol Table
        </h3>
      </div>

      <div class="overflow-auto custom-scrollbar flex-1">
        <table class="table table-xs table-pin-rows w-full">
          <thead>
            <tr>
              <th>Symbol Name</th>
              <th>Type</th>
              <th>Data Type</th>
              <th>Offset / Value</th>
              <th>Segment</th>
            </tr>
          </thead>
          <tbody>
            {#if glyphStore.parserResult.symbol_table.length === 0}
                <tr>
                    <td colspan="5" class="text-center text-base-content/50 py-4">
                        No symbols defined yet.
                    </td>
                </tr>
            {:else}
                {#each glyphStore.parserResult.symbol_table as sym}
                <tr class="hover:bg-base-200">
                    <!-- Name -->
                    <td class="font-bold text-primary font-mono">{sym.name}</td>
                    
                    <!-- Type (Variable/Label) -->
                    <td>
                        <span class="badge badge-ghost badge-xs">{sym.type_}</span>
                    </td>

                    <!-- Data Size (DB/DW) -->
                    <td>
                        {#if sym.data_type !== "None"}
                            <span class="font-mono text-xs">{sym.data_type}</span>
                        {:else}
                            <span class="text-base-content/30">-</span>
                        {/if}
                    </td>

                    <!-- Value (Offset) -->
                    <td class="font-mono text-secondary">
                        {toHex(sym.value)}h
                    </td>

                    <!-- Segment -->
                    <td>
                        <span class="badge badge-outline badge-xs opacity-70">
                            {sym.segment}
                        </span>
                    </td>
                </tr>
                {/each}
            {/if}
          </tbody>
        </table>
      </div>
    </div>

  {/if}
</div>