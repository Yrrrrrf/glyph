<!-- src/routes/+layout.svelte -->
<script lang="ts">
	import { onMount } from 'svelte';
	import { init } from '$lib/index';
	import '../app.css';
	// Get the children render snippet from props using Svelte 5's $props rune.
	let { children } = $props();

	// `onMount` is the correct lifecycle function for client-side only operations
	// like initializing a WebAssembly module. It runs once after the component
	// is first rendered in the browser.
	onMount(async () => {
		await init();
	});
</script>

<svelte:head>
	<title>Glyph</title>
	<!-- <link rel="icon" href={'/img/mineral.png'} /> -->
	<link rel="icon" href={'/img/diamond.png'} />
</svelte:head>

<!-- todo: Enhance these to be imported from the main wasm.ts script -->

	<!-- App Content: Once ready, render the actual page content. -->
<div class="app-container">
	{@render children()}
</div>
