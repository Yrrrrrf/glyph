<script lang="ts">
    import ThemeSelector from "./ThemeSelector.svelte";

	let { onAnalyze } = $props<{
		onAnalyze: () => void;
	}>();
</script>

<header class="bg-base-200 p-4 flex items-center justify-between">
	<div class="flex items-center space-x-2">
		<h1 class="text-xl font-bold">Glyph</h1>
		<ThemeSelector />
	</div>
	<div class="flex space-x-2">
		<button class="btn btn-primary flex items-center" onclick={onAnalyze}>
			<svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
			</svg>
			Analyze
		</button>
	</div>
</header>