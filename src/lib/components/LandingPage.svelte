<!-- src/lib/components/LandingPage.svelte -->
<script lang="ts">
  import FileInput from './FileInput.svelte';
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
</script>

<div class="flex flex-col items-center justify-center h-full p-8 gap-8">
  <div class="text-6xl mb-4">🏺</div>
  <h2 class="text-2xl font-bold">Welcome to Glyph</h2>
  <p class="text-base-content/70 max-w-md text-center">
    A modern assembly analyzer powered by Rust & WebAssembly. Load a file to begin.
  </p>
  
  <div class="w-full max-w-md">
    <FileInput onFileLoaded={glyphStore.loadFile} />
  </div>
  
  <div class="text-xs text-base-content/50">
    Built with Svelte 5, Tailwind 4, and Rust 🦀
  </div>
</div>