This is the perfect mindset. You are essentially asking to move from a
**"Brittle Compiler"** (fails on first error) to a **"Fault-Tolerant Language
Server"** architecture.

This is how modern tools like `rust-analyzer` or `gopls` work. They don't crash
when you type a syntax error; they wrap it, analyze the rest of the file, and
present the error contextually.

Here is your **Advanced Technical Roadmap** to refactor Glyph into a resilient,
line-oriented assembler while keeping the modern Rust/WASM stack.

---

### 🏗️ Architectural Concept: The "Resilient Pipeline"

### 🛠️ Phase 0: The Baseline (Python reference)

1. **Initialize `uv` Project:** You will analyze the output from the
   `full_pipe_runner.py` and the implementation of the `ensamblador.py` scirpt.
   ```sh
   uv run full_pipe_runner.py ../static/x8086/comp/conversor-masm.asm
   # or
   uv run full_pipe_runner.py ../static/x8086/plantilla.asm
   ```

The output of this script is your **Ground Truth**. It shows how the current
Python implementation handles errors and outputs the "Phase 2 Table".

We will shift the mental model from **"Source -> Program"** to **"Source ->
List<LineResult>"**.

#### 1. Data Structure Evolution (The AST)

We stop enforcing that a Program is a list of valid Statements. Instead, a
Program is a list of _Lines_, and a Line can be Valid, Empty, or Garbage.

**The Strategy:**

- **Keep** your `Statement` enum (Instruction, Variable, etc.).
- **Wrap** it in a higher-order enum that represents the status of a line.

```rust
// The Conceptual AST
pub enum LineNode {
    // A fully parsed, syntactically correct statement
    Statement(Statement), 
    
    // A comment or empty line
    Empty,                
    
    // A syntax error (we couldn't parse it, but we captured the raw text)
    Error { 
        raw_content: String, 
        message: String 
    }, 
}

// The Output struct sent to Svelte
pub struct AnalysisResult {
    pub lines: Vec<LineNode>,
    pub symbol_table: Vec<SymbolRecord>,
}
```

#### 2. The Lexer: "Semantic Whitespace"

Standard compilers treat `\n` as whitespace. Assemblers treat `\n` as a
**delimiter**.

**The Technical Change:**

- **Tokenize Newlines:** Modify `lexer.rs` to stop ignoring newlines. Emit a
  specific `Token::Newline`.
- **The Synchronizer:** This `Newline` token becomes your **Synchronization
  Point**. When the parser gets confused (garbage input), it skips tokens until
  it hits a `Newline`, then resets its state for the next line.

#### 3. The Parser: "Panic Mode Recovery"

This is the most sophisticated part. We use `chumsky`'s error recovery
combinators to create a "Total Parser" (one that always succeeds).

**The Logic:**

1. Define a parser for a single line (`statement_parser`).
2. Wrap it in a recovery strategy:
   - _Try_ to parse `statement_parser`.
   - _If it fails_, consume tokens until `Token::Newline`.
   - _Yield_ a `LineNode::Error`.
3. The main parser becomes `line_parser.repeated()`.

**Why this is "Technical Excellence":** This allows you to render the "Phase 2
Table" perfectly.

- Line 1: `.code segment` -> `Statement(Segment)` -> "Correcta"
- Line 2: `invalid garbage` -> `Error(...)` -> "Incorrecta"
- Line 3: `MOV AX, BX` -> `Statement(Instruction)` -> "Correcta"

#### 4. The Validator: "The Linter Pass" (Contextual Logic)

The Python script mixes parsing (`re.match`) with logic (`if segment == CODE`).
We will separate them for cleanliness.

**The Strategy:** Create a dedicated `semantics/linter.rs` that takes the
`Vec<LineNode>` and runs the state machine.

- **State:** `Context { current_segment: SegmentType }`.
- **Rules Engine:**
  - Iterate through the lines.
  - Update `current_segment` if the line is a Segment Definition.
  - **Rule:** If `LineNode::Statement(Instruction)` AND
    `current_segment != Code` => **Transform** that node into a `SemanticError`
    (or attach an error flag).
  - **Rule:** If `LineNode::Statement(Variable)` AND `current_segment == Code`
    => Error.

This separation means your Parser handles **Grammar** (Syntax), and your Linter
handles **Rules** (Semantics). This allows you to easily switch rules later
(e.g., "Allow instructions in data" for a specific weird dialect) without
touching the parser.

#### 5. Phase 1 (Visuals): The "View Model" Pattern

For the Requirement A ("Display `.data segment` as one item"):

- **Do not** hack the Rust Lexer to merge tokens. That makes the compiler
  dumber.
- **Do** implement this in the `lib.rs` boundary layer or the Svelte component.

**The "View Model" Transformation:** Before sending `Vec<JsToken>` to the
frontend, run a light pass:

- If Token `i` is `.data` and Token `i+1` is `segment`:
  - Combine them into one `JsToken` for display.
- Otherwise, pass them through.

This keeps your backend clean/atomic (Advanced) while satisfying the specific
academic requirement (Compliance).

---

### 🗺️ The Step-by-Step Implementation Plan

We will execute this in order to maintain stability.

**Step 1: The Test Harness**

- Create a Rust test file that reads `ejemplo.asm` (from the Python inputs).
- Create a function that outputs the exact text table format the Python script
  does.
- _Goal:_ See where Rust fails vs Python currently.

**Step 2: Lexical Refactor**

- Update `lexer.rs` to emit `Token::Newline`.

**Step 3: Parser Resilience**

- Modify `parser.rs` to return `Vec<LineNode>`.
- Implement `recover_with(skip_until(...))` to handle syntax errors without
  crashing.
- _Checkpoint:_ You should be able to parse `ejemplo_errores.asm` and get a list
  of Mixed Success/Error nodes.

**Step 4: The Logic Pass**

- Implement `validator.rs` state machine.
- Add the logic: "Instructions only valid in Code segment", "Variables only in
  Data/Stack".
- Update the Output struct to carry "Status" (Correcta/Incorrecta) strings.

**Step 5: Addressing & Encoding (The Finisher)**

- Update `encoder.rs` to calculate offsets based on the `LineNode` list.
- Implement the `0x0250` reset rule.
- Implement the specific opcodes for Team 2 (hardcode the hex values if needed,
  or build a mini-encoder (but do not add them for team 2, just add it for the
  CURRENT TOKENS, wo won't add any new ones, that are specfici for that
  project)).
