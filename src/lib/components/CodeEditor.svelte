<script lang="ts">
	type HighlightInfo = { line: number; element: string } | null;

	let { code, highlight } = $props<{
		code: string;
		highlight: HighlightInfo;
	}>();

	let lines = $derived(code.split('\n'));
</script>

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full">
	<div class="font-mono text-sm">
		{#each lines as line, i}
			<div class="flex hover:bg-base-200" class:bg-base-300={highlight?.line === i + 1}>
				<div class="w-12 flex-shrink-0 px-3 py-1 text-right text-gray-500 select-none">
					{i + 1}
				</div>
				<div class="px-3 py-1 flex-grow">
					{line}
				</div>
			</div>
		{/each}
	</div>
</div>