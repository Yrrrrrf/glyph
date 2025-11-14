<!-- src/lib/components/ErrorBanner.svelte -->
<script lang="ts">
  import { glyphStore } from '$lib/stores/glyphStore.svelte';
  
  let show = $state(false);
  
  $effect(() => {
    if (glyphStore.error) {
      show = true;
      const timer = setTimeout(() => {
        show = false;
        glyphStore.clearError();
      }, 5000);
      return () => clearTimeout(timer);
    }
  });
</script>

{#if show && glyphStore.error}
  <div class="alert alert-error rounded-none border-0 border-b border-error px-6 py-2 z-50">
    <div class="flex items-center gap-3 w-full max-w-7xl mx-auto">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor">
        <path fill-rule="evenodd" d="M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003zM12 8.25a.75.75 0 01.75.75v3.75a.75.75 0 01-1.5 0V9a.75.75 0 01.75-.75zm0 8a.75.75 0 100-1.5.75.75 0 000 1.5z" clip-rule="evenodd"/>
      </svg>
      <span class="flex-1 text-sm">{glyphStore.error.message}</span>
      <button class="btn btn-xs btn-ghost" onclick={() => {
        show = false;
        glyphStore.clearError();
      }}>
        ✕
      </button>
    </div>
  </div>
{/if}