<script lang="ts">
	// Import the clean, wrapped function directly from $lib (which points to src/lib/index.ts)
	import { greet } from '$lib/wasm';

	let isLoading = false;

</script>

<div class="p-8">
	<h1 class="text-4xl font-bold">Welcome to Glyph!</h1>
	<p class="mt-2">The web-powered assembler toolkit.</p>
	<div class="card bg-base-200 shadow-xl p-4 mt-8 max-w-md">
		<h2 class="card-title">Test the Rust ↔ Svelte Bridge</h2>
		<p>Click the button to call a function written in Rust and compiled to WebAssembly.</p>
		<div class="card-actions justify-end mt-4">
			<button class="btn btn-primary" onclick={() => greet('SvelteKit and Rust')} disabled={isLoading}>
				Greet from Rust!
			</button>
		</div>
	</div>
</div>