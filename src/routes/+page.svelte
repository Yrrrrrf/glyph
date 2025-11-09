<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import CodeEditor from '$lib/components/CodeEditor.svelte';
	import AnalysisPanel from '$lib/components/AnalysisPanel.svelte';

	// Sample initial data for demonstration
	const initialSampleCode = `; Sample assembly code
section .data
    msg db 'Hello, World!', 0

section .text
    global _start

_start:
    ; write system call
    mov rax, 1      ; sys_write
    mov rdi, 1      ; stdout
    mov rsi, msg    ; message to write
    mov rdx, 13     ; number of bytes
    syscall

    ; exit system call
    mov rax, 60     ; sys_exit
    mov rdi, 0      ; exit status
    syscall`;

	// Mock tokens data for demonstration
	const mockTokens = [
		{ line: 1, element: 'section', type: 'directive' },
		{ line: 1, element: '.data', type: 'section' },
		{ line: 2, element: 'msg', type: 'label' },
		{ line: 2, element: 'db', type: 'directive' },
		{ line: 4, element: 'section', type: 'directive' },
		{ line: 4, element: '.text', type: 'section' },
		{ line: 5, element: 'global', type: 'directive' },
		{ line: 5, element: '_start', type: 'label' },
		{ line: 7, element: '_start:', type: 'label' },
		{ line: 9, element: 'mov', type: 'instruction' },
		{ line: 10, element: 'mov', type: 'instruction' },
		{ line: 11, element: 'mov', type: 'instruction' },
		{ line: 12, element: 'mov', type: 'instruction' },
		{ line: 13, element: 'syscall', type: 'instruction' },
		{ line: 15, element: 'mov', type: 'instruction' },
		{ line: 16, element: 'mov', type: 'instruction' },
		{ line: 17, element: 'syscall', type: 'instruction' },
	];

	// Holds the raw text from the .asm file.
	let sourceCode = $state(initialSampleCode);

	// Holds the array of tokens from the lexer.
	let analysisTokens = $state(mockTokens);

	// The 'bridge' between panels. Holds info on the hovered token.
	// It's null when nothing is hovered.
	let highlightedInfo = $state<{ line: number; element: string } | null>(null);

	// This function IS the callback. It directly modifies the state.
	function handleTokenHover(info: { line: number; element: string } | null) {
		highlightedInfo = info;
	}

	// This function will be passed to the Header.
	async function runAnalysis() {
		// Future integration point for Rust/WASM
		// const tokens = await wasm.analyze(sourceCode);
		// analysisTokens = tokens;
		console.log("Analysis triggered!");
	}
</script>

<div class="flex flex-col h-screen">
	<Header onAnalyze={runAnalysis} />
	<main class="flex flex-1 overflow-hidden p-4 gap-4">
		<div class="flex-1">
			<h2 class="text-lg font-semibold mb-2">Code Editor</h2>
			<CodeEditor
				code={sourceCode}
				highlight={highlightedInfo}
			/>
		</div>
		<div class="flex-1">
			<h2 class="text-lg font-semibold mb-2">Analysis Panel</h2>
			<AnalysisPanel
				tokens={analysisTokens}
				onTokenHover={handleTokenHover}
			/>
		</div>
	</main>
</div>