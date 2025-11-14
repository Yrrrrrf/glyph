<script lang="ts">
  let { onFileLoaded } = $props<{
    onFileLoaded: (content: string, filename: string) => void;
  }>();

  let error = $state<string | null>(null);

  function handleFileInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    
    error = null;
    
    if (!file) return;

    if (!file.name.toLowerCase().endsWith('.asm')) {
      error = `❌ Invalid file: "${file.name}" is not a .asm file`;
      input.value = '';
      return;
    }

    const reader = new FileReader();
    reader.onload = (e) => {
      const content = e.target?.result as string;
      onFileLoaded(content, file.name);
    };
    reader.readAsText(file);
  }
</script>

<div class="form-control w-full max-w-md">
    <span class="label-text font-semibold">Select Assembly File</span>
  <input 
    type="file" 
    class="file-input file-input-primary file-input-sm w-full" 
    onchange={handleFileInput}
    />
    <!-- accept=".asm" -->
  
  {#if error}
      <span class="label-text-alt text-error">{error}</span>
  {:else}
      <span class="label-text-alt text-base-content/60">
        Choose a *.asm script to analyze
      </span>
  {/if}
</div>