<!-- src/lib/components/FileInput.svelte -->
<script lang="ts">
  let { onFileLoaded } = $props<{
    onFileLoaded: (content: string, filename: string) => Promise<void>;
  }>();

  let error = $state<string | null>(null);

  async function handleFileInput(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    error = null;
    console.log('📄 FileInput selected:', file.name);

    // Validate
    if (!file.name.toLowerCase().endsWith('.asm')) {
      error = `Invalid file: "${file.name}" is not a .asm file`;
      console.error('❌ FileInput validation failed:', error);
      return;
    }

    const reader = new FileReader();
    reader.onload = async (e) => {
      const content = e.target?.result as string;
      console.log('📖 File read complete, length:', content.length);
      try {
        await onFileLoaded(content, file.name);
        console.log('✅ FileInput: onFileLoaded succeeded');
        input.value = ''; // Reset for re-upload
      } catch (err) {
        error = err instanceof Error ? err.message : 'Failed';
        console.error('❌ FileInput: onFileLoaded failed:', error);
      }
    };
    
    reader.readAsText(file);
  }
</script>

<div class="form-control w-full max-w-xs">
    <span class="label-text font-semibold">Select Assembly File</span>
  
  <input 
    type="file" 
    class="file-input file-input-primary file-input-xs w-full" 
    onchange={handleFileInput}
  />
  
  {#if error}
      <span class="label-text-alt text-error">{error}</span>
  {/if}
</div>