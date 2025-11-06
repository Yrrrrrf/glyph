<script lang="ts">
	import { onMount } from 'svelte';
	import init, { greet } from '../lib/pkg/glyph.js';

	let wasmReady = false;
	let message = 'Initializing WebAssembly module...';

	// onMount runs only in the browser after the component is rendered
	onMount(async () => {
		try {
			// Await the WASM initialization
			await init();
			wasmReady = true;
			message = 'WASM module loaded successfully!';
		} catch (e) {
			console.error('Error initializing WASM:', e);
			message = 'Failed to load WebAssembly module.';
		}
	});

	function handleGreetClick() {
		if (wasmReady) {
			// Now it's safe to call the greet function
			greet('SvelteKit');
		} else {
			alert('WASM is not ready yet!');
		}
	}
</script>

<h1>Welcome to SvelteKit + Rust!</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation.</p>

<div class="card bg-base-200 shadow-xl p-4 mt-4">
	<p>{message}</p>
	<div class="card-actions justify-end mt-2">
		<!-- Disable the button until WASM is ready -->
		<button class="btn btn-primary" on:click={handleGreetClick} disabled={!wasmReady}>
			Call Rust Greet Function
		</button>
	</div>
</div>