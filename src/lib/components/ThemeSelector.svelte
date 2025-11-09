<script lang="ts">
    interface ThemeConfig {
        name: string;
        icon: string;
        description?: string;
    }

    // todo: Find a way to handle this dynamically from tailwind config or daisyui
    const STATIC_THEMES: ThemeConfig[] = [
        { name: "light", icon: "🌞", description: "Clean, light mode appearance" },
        { name: "dark", icon: "🌙", description: "Sleek dark mode interface" },
        { name: "cupcake", icon: "🧁", description: "Sweet pastel colors" },
        { name: "bumblebee", icon: "🐝", description: "Sharp black and yellow" },
        { name: "emerald", icon: "💚", description: "Lush green and neutral tones" },
        { name: "corporate", icon: "🏢", description: "Professional and modern" },
        { name: "synthwave", icon: "🌆", description: "Retro-futuristic vibrant style" },
        { name: "retro", icon: "📺", description: "Nostalgic vintage look" },
        { name: "cyberpunk", icon: "🤖", description: "High-tech, neon aesthetic" },
        { name: "valentine", icon: "💝", description: "Soft romantic theme" },
        { name: "halloween", icon: "🎃", description: "Spooky orange and black" },
        { name: "garden", icon: "🌷", description: "Fresh natural colors" },
        { name: "forest", icon: "🌲", description: "Deep woodland tones" },
        { name: "aqua", icon: "💧", description: "Cool aquatic colors" },
        { name: "lofi", icon: "🎵", description: "Minimalist and calm" },
        { name: "pastel", icon: "🎨", description: "Soft pastel palette" },
        { name: "fantasy", icon: "🔮", description: "Mystical and enchanting" },
        { name: "wireframe", icon: "📝", description: "Simple black and white" },
        { name: "black", icon: "⚫", description: "Monochrome dark theme" },
        { name: "luxury", icon: "👑", description: "Elegant gold and black" },
        { name: "dracula", icon: "🧛", description: "Dark vampire-inspired" },
        { name: "cmyk", icon: "🖨️", description: "Print-inspired colors" },
        { name: "autumn", icon: "🍂", description: "Warm autumn colors" },
        { name: "business", icon: "💼", description: "Clean business look" },
        { name: "acid", icon: "🧪", description: "Bright neon colors" },
        { name: "lemonade", icon: "🍋", description: "Fresh citrus theme" },
        { name: "night", icon: "🌃", description: "Dark nighttime colors" },
        { name: "coffee", icon: "☕", description: "Warm coffee tones" },
        { name: "winter", icon: "❄️", description: "Cool winter colors" },
        { name: "dim", icon: "🔅", description: "Dimmed light theme" },
        { name: "nord", icon: "❄️", description: "Arctic color palette" },
        { name: "sunset", icon: "🌅", description: "Warm sunset colors" },
    ];

    let currentTheme = $state(getCurrentTheme());
    
    // Get the current theme from HTML data-theme attribute
    function getCurrentTheme(): string {
        if (typeof document !== 'undefined') {
            return document.documentElement.getAttribute('data-theme') || 'night';
        }
        return 'night';
    }

    // Create a map of theme name to config for quick lookup
    const themeMap = new Map(STATIC_THEMES.map(theme => [theme.name, theme]));

    // Get current theme config
    $effect(() => {
        // Update the current icon when theme changes
        if (typeof document !== 'undefined') {
            document.documentElement.setAttribute('data-theme', currentTheme);
        }
    });

    // Computed property for current theme config
    let themeConfig = $derived(themeMap.get(currentTheme) || { 
        name: currentTheme, 
        icon: "🎨",
        description: "Custom theme" 
    });

    // Handler to change theme
    function changeTheme(themeName: string) {
        currentTheme = themeName;
    }
</script>

<div class="dropdown dropdown-bottom">
    <div 
        tabindex="0" 
        role="button" 
        class="btn btn-ghost btn-circle"
        aria-label="Select theme"
    >
        <span class="flex items-center justify-center">
            <span class="text-lg">{themeConfig.icon}</span>
        </span>
    </div>

    <div class="dropdown-content z-30 menu p-3 bg-base-200 border border-base-300 rounded-box shadow-xl mt-2 w-64 max-h-96 overflow-y-auto">
        <ul class="menu bg-base-100 rounded-box w-full">
            {#each STATIC_THEMES as theme}
                <li>
                    <button 
                        class="flex items-center gap-2 hover:bg-base-200 active:bg-base-300 transition-colors duration-200" 
                        onclick={() => changeTheme(theme.name)}
                    >
                        <div class="w-5 flex justify-center">
                            {#if currentTheme === theme.name}
                                <div class="text-sm text-primary">✓</div>
                            {/if}
                        </div>
                        <span class="text-xl">{theme.icon}</span>
                        <span class="capitalize">{theme.name}</span>
                    </button>
                </li>
            {/each}
        </ul>
    </div>
</div>