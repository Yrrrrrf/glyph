<!-- src/lib/components/FileInput.svelte -->
<script lang="ts">
  import * as m from '$lib/paraglide/messages';

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

    // Validate extension
    if (!file.name.toLowerCase().endsWith('.asm')) {
      error = m.file_input_invalid_file({ fileName: file.name });
      console.error('❌ FileInput validation failed:', error);
      return;
    }

    const reader = new FileReader();
    
    reader.onload = async (e) => {
      let content = e.target?.result as string;
      
      // --- FIX: REMOVE REPLACEMENT CHARACTERS ---
      // This removes the  character which causes offset desyncs
      // between the Rust backend (byte-based) and JS frontend (char-based).
      content = content.replace(/\uFFFD/g, ""); 
      
      console.log('📖 File read complete, length:', content.length);
      
      try {
        await onFileLoaded(content, file.name);
        console.log('✅ FileInput: onFileLoaded succeeded');
        input.value = ''; // Reset for re-upload
      } catch (err) {
        error = err instanceof Error ? err.message : m.file_input_failed();
        console.error('❌ FileInput: onFileLoaded failed:', error);
      }
    };
    
    // Default reads as UTF-8. If the file is legacy ANSI, it produces .
    // By cleaning the result above, we solve the sync issue.
    reader.readAsText(file);
  }
</script>

<div class="form-control w-full max-w-xs">
  <div class="label-text font-semibold text-center w-full mb-2">{m.file_input_select_assembly_file()}</div>
    
  <input 
    type="file" 
    class="file-input file-input-primary file-input-lx w-full" 
    onchange={handleFileInput}
  />
  
  {#if error}
      <span class="label-text-alt text-error">{error}</span>
  {/if}
</div>