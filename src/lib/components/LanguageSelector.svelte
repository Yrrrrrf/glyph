<script lang="ts">
  import { locales, setLocale, getLocale } from '$lib/paraglide/runtime';
</script>

<div class="flex gap-2">
  {#each locales as lang}
    <button
      class="p-2 border rounded uppercase"
      class:bg-neutral-200={getLocale() === lang}
      class:font-bold={getLocale() === lang}
      onclick={() => setLocale(lang)}
    >
      {lang}
    </button>
  {/each}
</div>