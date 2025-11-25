<script lang="ts">
    import type { Snippet } from 'svelte';
    import * as m from '$lib/paraglide/messages';

    interface ThemeConfig {
        name: string;
        icon: string;
        description?: string;
    }

    // Props: Accept an optional icon snippet
    let { icon }: { icon?: Snippet } = $props();

    // Internal state for the visual checkmark selection
    // Defaulting to 'light', but you could initialize this from localStorage if needed
    let currentTheme = $state("light");

    const STATIC_THEMES: ThemeConfig[] = [
        { name: "light", icon: "🌞" },
        { name: "dark", icon: "🌙" },
        { name: "cupcake", icon: "🧁" },
        { name: "bumblebee", icon: "🐝" },
        { name: "emerald", icon: "💚" },
        { name: "corporate", icon: "🏢" },
        { name: "synthwave", icon: "🌆" },
        { name: "retro", icon: "📺" },
        { name: "cyberpunk", icon: "🤖" },
        { name: "valentine", icon: "💝" },
        { name: "halloween", icon: "🎃" },
        { name: "garden", icon: "🌷" },
        { name: "forest", icon: "🌲" },
        { name: "aqua", icon: "💧" },
        { name: "lofi", icon: "🎵" },
        { name: "pastel", icon: "🎨" },
        { name: "fantasy", icon: "🔮" },
        { name: "wireframe", icon: "📝" },
        { name: "black", icon: "⚫" },
        { name: "luxury", icon: "👑" },
        { name: "dracula", icon: "🧛" },
        { name: "cmyk", icon: "🖨️" },
        { name: "autumn", icon: "🍂" },
        { name: "business", icon: "💼" },
        { name: "acid", icon: "🧪" },
        { name: "lemonade", icon: "🍋" },
        { name: "night", icon: "🌃" },
        { name: "coffee", icon: "☕" },
        { name: "winter", icon: "❄️" },
        { name: "dim", icon: "🔅" },
        { name: "nord", icon: "❄️" },
        { name: "sunset", icon: "🌅" },
    ];
</script>

<div class="dropdown dropdown-bottom">
    <!-- Trigger Button -->
    <div 
        tabindex="0" 
        role="button" 
        class="btn btn-ghost btn-circle"
        aria-label={m.theme_selector_select_theme()}
    >
        <span class="flex items-center justify-center">
            {#if icon}
                <!-- Render passed icon snippet -->
                {@render icon()}
            {:else}
                <!-- Default Palette Icon -->
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="13.5" cy="6.5" r="2.5" />
                    <circle cx="17.5" cy="10.5" r="2.5" />
                    <circle cx="8.5" cy="7.5" r="2.5" />
                    <circle cx="6.5" cy="12.5" r="2.5" />
                    <path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z" />
                </svg>
            {/if}
        </span>
    </div>

    <!-- Dropdown Menu -->
    <div class="dropdown-content z-[1] menu p-2 bg-base-200 border border-base-300 rounded-box shadow-2xl mt-2 w-64 max-h-96 overflow-y-auto">
        <ul class="menu bg-base-100 rounded-box w-full">
            {#each STATIC_THEMES as theme}
                <li class="p-1">
                    <input 
                        type="radio" 
                        name="{theme}-button"
                        class="btn theme-controller join-item"
                        value={theme.name}
                        bind:group={currentTheme}
                        aria-label="{theme.icon} {theme.name}"
                    >
                </li>
            {/each}
        </ul>
    </div>
</div>