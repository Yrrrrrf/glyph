Of course. I've analyzed the `rune-lab` project and understand the Svelte 5 patterns you're using. You are correct—`createEventDispatcher` is the old way. Svelte 5 strongly favors passing callback functions as props, which creates a cleaner and more explicit data flow.

Here is the updated architectural plan, fully revised for **Svelte 5**, using runes and the modern callback pattern.

---

### **Core Architectural Principle: Conductor with Unidirectional Data Flow & Callback Props**

Our architecture remains centered on a "Conductor" component (`+page.svelte`) that owns and manages all application state. However, the communication mechanism is now modernized for Svelte 5:

*   **Data Flows Down:** The Conductor passes state (like `sourceCode` and `highlightedInfo`) down to child components as props.
*   **Actions Flow Up via Callbacks:** Child components no longer dispatch events. Instead, they receive *functions* as props (e.g., `onAnalyze`, `onTokenHover`). When a user interacts with a child component, it calls the function it was given, passing data directly back up to the Conductor to modify the state.

**Why this Svelte 5 approach is better:**

*   **Explicit Contracts:** The component's props (`$props`) explicitly define both the data it needs and the actions it can perform. There's no "magic" event dispatching.
*   **Type Safety:** It's much easier to create strongly-typed contracts for callback functions than for event payloads.
*   **Simplicity:** It removes the need for `createEventDispatcher`, reducing boilerplate and aligning Svelte with patterns common in other modern frameworks like React.

---

### **Component Blueprint: Svelte 5 Implementation**

Here is the detailed, updated plan for each component.

#### **1. The Conductor: `src/routes/+page.svelte`**

*   **What it does:** The brain of the application. It owns all state and defines the logic for how that state can be changed.
*   **Why it exists:** To create a single, authoritative source of truth, making the application's behavior predictable and easy to debug.
*   **How it will be implemented in Svelte 5:**
    *   **State Management (Runes):** All reactive state is declared using the `$state` rune.
        ```svelte
        <script lang="ts">
            import Header from '$lib/components/Header.svelte';
            import CodeEditor from '$lib/components/CodeEditor.svelte';
            import AnalysisPanel from '$lib/components/AnalysisPanel.svelte';

            // Holds the raw text from the .asm file.
            let sourceCode = $state(initialSampleCode);

            // Holds the array of tokens from the lexer.
            let analysisTokens = $state(mockTokens);

            // The 'bridge' between panels. Holds info on the hovered token.
            // It's null when nothing is hovered.
            let highlightedInfo = $state<{ line: number; element: string } | null>(null);
        </script>
        ```
    *   **Logic (Callback Functions):** It defines the functions that will be passed down to children to handle interactions.
        ```svelte
        <script lang="ts">
            // ... (state declarations) ...

            // This function IS the callback. It directly modifies the state.
            function handleTokenHover(info: { line: number; element: string } | null) {
                highlightedInfo = info;
            }

            // This function will be passed to the Header.
            async function runAnalysis() {
                // Future integration point for Rust/WASM
                // const tokens = await wasm.analyze(sourceCode);
                // analysisTokens = tokens;
                console.log("Analysis triggered!");
            }
        </script>
        ```
    *   **Component Composition (Passing Callbacks):** The template wires everything together, passing state down as props and functions down as callback handlers.
        ```svelte
        <Header onAnalyze={runAnalysis} />
        <main class="flex flex-1">
            <CodeEditor
                code={sourceCode}
                highlight={highlightedInfo}
            />
            <AnalysisPanel
                tokens={analysisTokens}
                onTokenHover={handleTokenHover}
            />
        </main>
        ```

#### **2. The Action Bar: `src/lib/components/Header.svelte`**

*   **What it does:** Renders the static top bar. Its only job is to tell the Conductor when the "Analyze" button is clicked.
*   **Why it's a separate component:** Encapsulation and reusability. It keeps the main page clean.
*   **How it will be implemented in Svelte 5:**
    *   **Props (`$props` rune):** It declares that it expects to receive an `onAnalyze` function as a prop.
        ```svelte
        <script lang="ts">
            let { onAnalyze } = $props<{
                onAnalyze: () => void;
            }>();
        </script>
        ```
    *   **Calling the Callback:** The button's `onclick` handler now calls the prop function directly. No dispatching needed.
        ```svelte
        <button
            class="flex items-center ... btn-accent ..."
            onclick={onAnalyze}
        >
            <Play class="h-4 w-4" />
            Analyze
        </button>
        ```

#### **3. The Code View: `src/lib/components/CodeEditor.svelte`**

*   **What it does:** Renders the source code, reacting to changes in the `highlight` prop to apply styling.
*   **Why it's a separate component:** Isolates the complex logic of code rendering and syntax highlighting.
*   **How it will be implemented in Svelte 5:**
    *   **Props (`$props` rune):** Declares its data dependencies clearly.
        ```svelte
        <script lang="ts">
            type HighlightInfo = { line: number; element: string } | null;

            let { code, highlight } = $props<{
                code: string;
                highlight: HighlightInfo;
            }>();
        </script>
        ```
    *   **Reactive Rendering (`$derived` rune):** We use `$derived` to efficiently re-calculate the `lines` array only when the `code` prop actually changes.
        ```svelte
        <script lang="ts">
            // ... (props declaration) ...
            let lines = $derived(code.split('\n'));
        </script>
        ```
    *   **Conditional Styling:** The template uses the modern, reactive approach to apply classes based on the `highlight` prop. The logic remains the same as it's already idiomatic Svelte.
        ```svelte
        {#each lines as line, i}
            <div class:bg-accent-10={highlight?.line === i + 1}>
                <!-- line number and syntax highlighted content -->
            </div>
        {/each}
        ```

#### **4. The Analysis View: `src/lib/components/AnalysisPanel.svelte`**

*   **What it does:** Renders the token table and, most importantly, calls the `onTokenHover` callback when the user's mouse enters or leaves a row.
*   **Why it's a separate component:** Follows the Single Responsibility Principle by focusing solely on displaying analysis data.
*   **How it will be implemented in Svelte 5:**
    *   **Props (`$props` rune):** It declares its dependency on the `tokens` data and the `onTokenHover` callback function.
        ```svelte
        <script lang="ts">
            type Token = { line: number; element: string; type: string };
            type HighlightInfo = { line: number; element: string } | null;

            let { tokens, onTokenHover } = $props<{
                tokens: Token[];
                onTokenHover: (info: HighlightInfo) => void;
            }>();
        </script>
        ```
    *   **Calling the Callback (The Key Interaction):** The table rows directly invoke the `onTokenHover` function passed down from the Conductor. This completes the reactive loop.
        ```svelte
        <tbody>
            {#each tokens as token (token.line + token.element)}
                <tr
                    onmouseenter={() => onTokenHover({ line: token.line, element: token.element })}
                    onmouseleave={() => onTokenHover(null)}
                >
                    <td>{token.line}</td>
                    <td>{token.element}</td>
                    <td>{token.type}</td>
                </tr>
            {/each}
        </tbody>
        ```

This updated blueprint fully embraces the Svelte 5 paradigm. It's cleaner, more explicit, and provides a robust foundation for building out the rest of your Glyph application and integrating the Rust/WASM core.
