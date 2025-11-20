# **Glyph Project: Svelte 5 Coding Standards & Best Practices**

## **1. Core Philosophy: Clarity and Predictability**

Our primary goal is to write code that is easy to understand, debug, and maintain. We will achieve this by adhering to a strict, unidirectional data flow powered by Svelte 5's runes. Every component should have a clear and explicit contract for the data it receives and the actions it can perform.

---

## **2. The Conductor Architectural Pattern**

This is the most important pattern we will follow.

*   **What it is:** A single, top-level component (usually a route like `+page.svelte`) acts as the "Conductor." This component is the **only** place where application state is managed and mutated.
*   **Why we use it:** It creates a **Single Source of Truth**. We always know where to find the state and the logic that changes it. This eliminates bugs caused by state being scattered across multiple, unrelated components.
*   **How it works:**
    1.  The Conductor defines all reactive state using the `$state` rune.
    2.  The Conductor passes this state **down** to child components as props.
    3.  The Conductor defines functions that modify its state. It passes these functions **down** to child components as callback props.
    4.  Child components are "dumb." They only display the data they are given and call the functions they are given. They **never** manage their own complex state.

---

## **3. The Rune System: Our Core Toolset**

Runes are the foundation of reactivity in Svelte 5. We will use them exclusively for managing state and side effects.

### **`$state` - The Source of Truth**

*   **What:** Declares a piece of reactive state. When its value changes, any part of the UI that depends on it will automatically update.
*   **When to use:** For any value that can change over time and should trigger a UI update. This includes user input, API data, UI status (e.g., `isModalOpen`), etc.
*   **Syntax:**
    ```svelte
    <script lang="ts">
        // In the Conductor component
        let sourceCode = $state("");
        let analysisTokens = $state<Token[]>([]);
        let highlightedLine = $state<number | null>(null);
    </script>
    ```

### **`$props` - The Component's Public API**

*   **What:** Defines the properties (props) that a component receives from its parent. This is the component's contract.
*   **When to use:** In **every child (presentation) component**. This is how they receive data and callback functions.
*   **Syntax:** We will always destructure props for cleaner access.
    ```svelte
    <script lang="ts">
        // In a child component like AnalysisPanel.svelte
        type Token = { line: number; element: string; type: string };

        let { tokens, onTokenHover } = $props<{
            tokens: Token[];
            onTokenHover: (info: { line: number; element: string } | null) => void;
        }>();
    </script>
    ```

### **`$derived` - For Computed Values**

*   **What:** Creates a reactive value that is derived from one or more other reactive values (`$state` or other `$derived` values). It recalculates automatically when its dependencies change.
*   **When to use:** To avoid cluttering your templates with logic. Use it for filtering lists, concatenating strings, calculating totals, or any value that is based on other state.
*   **Syntax:**
    ```svelte
    <script lang="ts">
        let lines = $derived(sourceCode.split('\n'));
        let hasTokens = $derived(analysisTokens.length > 0);
    </script>
    ```

### **`$effect` - For Side Effects**

*   **What:** Runs a piece of code that interacts with the "outside world" (e.g., the DOM, `console.log`, API calls) whenever its dependencies change.
*   **When to use:**
    *   Logging a value to the console when it changes.
    *   Triggering an API call when a state variable (like a user ID) changes.
    *   Manually interacting with a non-Svelte library or a DOM element.
*   **Note:** We prefer `$effect` over `onMount` for any logic that needs to react to state changes. `onMount` is reserved for one-time setup code that does not depend on reactive state.

    ```svelte
    <script lang="ts">
        let { userId } = $props<{ userId: string }>();

        // This effect will re-run automatically whenever the userId prop changes.
        $effect(() => {
            console.log(`User ID changed to: ${userId}`);
            // In a real app, you might fetch user data here.
            // const data = await fetch(`/api/users/${userId}`);
        });
    </script>
    ```

---

## **4. Communication Pattern: Callbacks, Not Events**

> **Golden Rule:** We **DO NOT** use `createEventDispatcher` or `dispatch()`.

*   **Why:** The callback prop pattern is more explicit, type-safe, and aligns with Svelte 5's design philosophy.
*   **How:** A parent component passes a function to a child component via a prop. The child component calls this function to notify the parent of an interaction.

### **The "Golden Pattern" Example**

This simple example demonstrates our entire communication architecture.

**`Parent.svelte` (The Conductor)**

```svelte
<script lang="ts">
    import Child from './Child.svelte';

    let count = $state(0);

    // This is the logic.
    function increment() {
        count += 1;
    }
</script>

<main>
    <h1>The Parent</h1>
    <!-- The increment function is passed down as a prop -->
    <Child currentCount={count} onIncrement={increment} />
</main>
```

**`Child.svelte` (The Presentation Component)**

```svelte
<script lang="ts">
    // The child declares its expected props, including the callback function.
    let { currentCount, onIncrement } = $props<{
        currentCount: number;
        onIncrement: () => void;
    }>();
</script>

<div class="card">
    <p>Current count from parent: {currentCount}</p>
    <!-- The child calls the function it was given. -->
    <button onclick={onIncrement}>
        Increment from Child
    </button>
</div>
```

---

## **5. Directory Structure & Naming**

*   **Components:** All reusable components will live in `src/lib/components/`.
*   **Sub-directories:** We will organize components by function (e.g., `layout/`, `ui/`, `features/`).
*   **File Naming:** Component files will always be `PascalCase.svelte` (e.g., `AnalysisPanel.svelte`).

## **6. Styling**

*   We will use **TailwindCSS** and **DaisyUI** utility classes for all styling.
*   `<style>` blocks should be avoided unless absolutely necessary for a complex, component-specific animation or structure that cannot be achieved with utilities. This keeps our styling consistent and co-located with the markup.
