<!-- src/lib/components/ParserView.svelte -->
<script lang="ts">
  import { glyphStore, getTokenTextClass } from '$lib/stores/glyphStore.svelte';
  import type { WasmToken } from '$lib/types/tokenTypes.svelte';
  import * as m from '$lib/paraglide/messages';

  const toHex = (num: number) => {
    return "0x" + num.toString(16).toUpperCase().padStart(4, '0');
  };

  // Helper to parse the error string from Rust
  function parseError(fullMsg: string) {
    let type = 'ERR';
    let text = fullMsg;
    let badgeClass = 'badge-error';

    if (fullMsg.startsWith('[LEX]')) {
        type = 'LEXER';
        text = fullMsg.replace('[LEX]', '').trim();
        badgeClass = 'badge-warning text-warning-content'; // Yellow for Lexer
    } else if (fullMsg.startsWith('[PAR]')) {
        type = 'PARSER';
        text = fullMsg.replace('[PAR]', '').trim();
        badgeClass = 'badge-error text-error-content';   // Red for Parser
    } else if (fullMsg.startsWith('[SEM]')) {
        type = 'LOGIC';
        text = fullMsg.replace('[SEM]', '').trim();
        badgeClass = 'badge-secondary text-secondary-content'; // Purple for Semantics
    }

    return { type, text, badgeClass };
  }

  // --- HIGHLIGHTING & FILTERING LOGIC ---
  
  function escapeHtml(text: string): string {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
  }

  // Removes comments by cutting text at the first ';'
  function processComments(text: string): string {
    const commentIdx = text.indexOf(';');
    if (commentIdx === -1) return escapeHtml(text);
    return escapeHtml(text.slice(0, commentIdx));
  }

  // Determines if a line should be shown.
  // Returns FALSE if the line is empty, whitespace only, or just a comment.
  function isLineVisible(text: string): boolean {
    const commentIdx = text.indexOf(';');
    const codePart = commentIdx === -1 ? text : text.slice(0, commentIdx);
    return codePart.trim().length > 0;
  }

  function highlightLine(lineText: string, lineNum: number): string {
    const tokens = glyphStore.lexerResult?.filter((t: WasmToken) => t.line === lineNum) || [];
    // Ensure tokens are sorted by position
    tokens.sort((a, b) => a.start - b.start);

    let html = "";
    let cursor = 0;

    for (const token of tokens) {
        // Find the token text in the current line relative to the cursor.
        const relativeStart = lineText.indexOf(token.element, cursor);

        if (relativeStart === -1) continue; 

        // 1. Text before the token (whitespace)
        const preText = lineText.slice(cursor, relativeStart);
        html += processComments(preText);

        // 2. The Token itself
        // Note: We assume tokens don't contain comments inside them
        const colorClass = getTokenTextClass(token.category);
        html += `<span class="${colorClass}">${escapeHtml(token.element)}</span>`;

        // Advance cursor
        cursor = relativeStart + token.element.length;
    }

    // 3. Remaining text
    const remainder = lineText.slice(cursor);
    html += processComments(remainder);

    return html;
  }
</script>

<div class="flex flex-col h-full gap-4">
  
  {#if !glyphStore.parserResult}
    <div class="flex flex-col items-center justify-center h-full opacity-50">
      <div class="loading loading-spinner loading-lg text-primary"></div>
      <p class="mt-4 text-sm font-bold text-base-content/50">{m.parser_view_no_analysis_data()}</p>
    </div>
  {:else}
    
    <!-- TOP HALF: LINE ANALYSIS -->
    <div class="flex-1 flex flex-col min-h-0 border border-base-300 rounded-lg bg-base-100 shadow-sm overflow-hidden">
      <!-- Header -->
      <div class="p-3 bg-base-200/50 backdrop-blur border-b border-base-300 flex justify-between items-center">
        <h3 class="font-bold text-sm flex items-center gap-2 text-base-content/70">
          <span>📋</span> {m.parser_view_line_analysis()}
        </h3>
        
        {#if glyphStore.parserResult.lines.filter(l => !l.is_correct).length > 0}
            <div class="badge badge-error gap-1 text-white font-bold animate-pulse">
                {m.parser_view_errors({ count: glyphStore.parserResult.lines.filter(l => !l.is_correct).length })}
            </div>
        {/if}
      </div>

      <div class="overflow-auto custom-scrollbar flex-1">
        <table class="table table-xs table-pin-rows w-full">
          <thead>
            <tr class="bg-base-100 text-base-content/50">
              <th class="w-16 text-center text-primary font-bold">Addr</th>
              <th class="w-12 font-normal text-center">{m.parser_view_ln()}</th>
              <th class="w-32 text-center text-secondary font-bold">Opcode</th>
              <th class="font-normal">{m.parser_view_instruction_source()}</th>
              <th class="w-24 text-center font-normal">{m.parser_view_status()}</th>
              <th class="font-normal">{m.parser_view_details()}</th>
            </tr>
          </thead>
          <tbody>
            {#each glyphStore.parserResult.lines as line (line.line_number)}
              <!-- Only render if the line has content (after hiding comments) or if it has an error -->
              {#if isLineVisible(line.instruction) || !line.is_correct}
                <tr class="group transition-colors border-l-4"
                    class:bg-error={!line.is_correct}
                    class:bg-opacity-5={!line.is_correct}
                    class:border-error={!line.is_correct}
                    class:border-transparent={line.is_correct}
                    class:hover:bg-base-200={line.is_correct}
                >
                    <!-- Address Column -->
                    <td class="font-mono text-xs text-primary text-center select-none align-middle opacity-80">
                        {#if line.address}
                           {line.address}
                        {/if}
                    </td>

                    <!-- Line Number -->
                    <td class="font-mono text-base-content/40 text-center select-none align-middle">
                        {line.line_number}
                    </td>

                    <!-- Opcode Column -->
                    <td class="font-mono text-[10px] text-secondary font-bold text-center select-none align-middle tracking-tight">
                        {#if line.machine_code}
                           {line.machine_code}
                        {/if}
                    </td>
                    
                    <!-- Instruction Text (Highlighted, No Comments) -->
                    <td class="font-mono font-medium whitespace-pre text-sm align-middle">
                    {@html highlightLine(line.instruction, line.line_number)}
                    </td>

                    <!-- Status Badge -->
                    <td class="text-center">
                    {#if !line.is_correct}
                        <div class="tooltip" data-tip="Error">
                            <span class="badge badge-xs badge-error font-bold text-white shadow-sm">ERR</span>
                        </div>
                    {:else}
                        <span class="opacity-0 group-hover:opacity-20 text-success text-xs font-bold transition-opacity">OK</span>
                    {/if}
                    </td>

                    <!-- Error Message (Enhanced) -->
                    <td class="text-xs font-medium py-2 align-middle">
                    {#if line.error_message}
                        {@const err = parseError(line.error_message)}
                        <div class="flex items-center gap-2">
                            <!-- ERROR TYPE BADGE -->
                            <span class="badge badge-sm font-bold border-none h-5 {err.badgeClass}">
                                {err.type}
                            </span>
                            
                            <!-- ERROR TEXT -->
                            <span class="text-base-content/80 font-mono text-[11px]">
                                {err.text}
                            </span>
                        </div>
                    {/if}
                    </td>
                </tr>
              {/if}
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <!-- BOTTOM HALF: SYMBOL TABLE -->
    <div class="h-1/3 flex flex-col min-h-0 border border-base-300 rounded-lg bg-base-100 shadow-sm overflow-hidden">
      <!-- ... Header ... -->
      <div class="p-3 bg-base-200/50 backdrop-blur border-b border-base-300">
        <h3 class="font-bold text-sm flex items-center gap-2 text-base-content/70">
          <span>🔑</span> {m.parser_view_symbol_table()}
        </h3>
      </div>

      <div class="overflow-auto custom-scrollbar flex-1">
        <table class="table table-xs table-pin-rows w-full">
          <thead>
            <tr class="bg-base-100 text-base-content/50">
              <th class="font-normal">{m.parser_view_symbol_name()}</th>
              <th class="font-normal">{m.parser_view_type()}</th>
              <th class="font-normal">{m.parser_view_data_type()}</th>
              <th class="font-normal font-mono text-right pr-8">{m.parser_view_offset_value()}</th>
              <th class="font-normal text-right">{m.parser_view_segment()}</th>
            </tr>
          </thead>
          <tbody>
            {#if glyphStore.parserResult.symbol_table.length === 0}
                <tr>
                    <td colspan="5" class="text-center text-base-content/30 py-8 italic">
                        {m.parser_view_no_symbols_defined()}
                    </td>
                </tr>
            {:else}
                {#each glyphStore.parserResult.symbol_table as sym}
                <tr class="hover:bg-base-200 transition-colors">
                    <td class="font-bold text-primary font-mono pl-4">{sym.name}</td>
                    <td>
                        <span class="badge badge-sm border-none font-semibold
                            {sym.type_ === 'Variable' ? 'bg-purple-100 text-purple-800 dark:bg-purple-900/30 dark:text-purple-300' : ''}
                            {sym.type_ === 'Label' ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300' : ''}
                            {sym.type_ === 'Constant' ? 'bg-orange-100 text-orange-800 dark:bg-orange-900/30 dark:text-orange-300' : ''}
                        ">
                            {sym.type_}
                        </span>
                    </td>
                    <td>
                        {#if sym.data_type !== "None"}
                            <span class="font-mono text-xs opacity-70">{sym.data_type}</span>
                        {:else}
                            <span class="text-base-content/20">-</span>
                        {/if}
                    </td>
                    <td class="font-mono text-secondary text-right pr-8">
                        {toHex(sym.value)}h
                    </td>
                    <td class="text-right pr-4">
                        <span class="text-xs font-mono opacity-50 uppercase">{sym.segment}</span>
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