<script lang="ts">
	type Token = { line: number; element: string; type: string };
	type HighlightInfo = { line: number; element: string } | null;

	let { tokens, onTokenHover } = $props<{
		tokens: Token[];
		onTokenHover: (info: HighlightInfo) => void;
	}>();
</script>

<div class="bg-base-100 border border-base-300 rounded-lg overflow-auto h-full">
	<table class="table table-xs">
		<thead>
			<tr>
				<th>Line</th>
				<th>Element</th>
				<th>Type</th>
			</tr>
		</thead>
		<tbody>
			{#each tokens as token (token.line + token.element)}
				<tr
					onmouseenter={() => onTokenHover({ line: token.line, element: token.element })}
					onmouseleave={() => onTokenHover(null)}
					class="hover"
				>
					<td>{token.line}</td>
					<td>{token.element}</td>
					<td>{token.type}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>