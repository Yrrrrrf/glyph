<!-- svelte-ignore a11y_missing_attribute -->
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { init } from '$lib/index';
  import '../app.css';
  
  let { children } = $props();
  
  onMount(async () => {
    await init();
  });
</script>

<svelte:head>
  <title>Glyph</title>
  <link rel="icon" href={'/img/diamond.png'} />
</svelte:head>

<div class="app-container">
  {@render children()}
</div>