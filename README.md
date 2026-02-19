# Programming Languages — Assignment Collection

This repository collects my Programming Languages assignments for the semester (Spring 2026). Work is written mainly in Rust and focuses on designing and implementing small compilers to learn the compilation pipeline end-to-end: parsing, AST construction, code generation, assembly emission, and linking.

## Adder (Week 1)

- **Purpose:** Build a minimal but complete compiler for a tiny expression language to understand how each compilation stage works together.
- **Learning objectives:**
  - Understand the basic structure of a compiler (lexer/parse → AST → codegen → link/run).
  - Parse S-expressions into an AST and model expressions in Rust.
  - Generate x86-64 assembly from abstract syntax and learn calling/return conventions.
  - Use Rust's type system and pattern matching to express compiler logic safely.
  - Use external tools (NASM, `ar`, and `rustc`/`cargo`) to assemble and link generated code.

**Layout**

- `src/` — Primary compiler code (parser, AST types, code generator, and `main.rs`).
- `runtime/` — Small Rust runtime (`start.rs`) that declares and calls into the generated symbol `our_code_starts_here` and prints the returned value.
- `test/` — Example `.snek` source files, generated `.s` assembly, and runnable artifacts/transcripts.
- `Makefile` — Targets to build generated assembly, archive it into `runtime/libour_code.a`, and produce test runners under `test/`.
- `Cargo.toml` — Rust manifest (dependencies such as `sexp` for parsing).

This layout keeps each week or assignment under its own header and folder so that every assignment can carry its source, tests, and transcript independently.

**How to run the provided tests (basic)**

The repository includes Makefile targets that mirror the assignment instructions. On Linux (example):

```bash
# generate assembly for a test (writes test/NAME.s)
make test/37.s

# assemble, archive, and build the test runner (produces test/37.run)
make test/37.run

# run the produced test program
./test/37.run

# inspect the generated assembly
cat test/37.s
```

Replace `37` with any test base name present in `test/` (for example, `add`, `negate`, `complex`). The Makefile handles NASM invocation and creation of `runtime/libour_code.a`.

Note: you do not strictly need to run every test locally. The provided tests are sub-folders under the `test/` folder and `transcript.txt` show the compiler runs and outputs for each test for grading. The transcript files contain the exact commands and terminal output used to demonstrate correctness.

## Future

More assignments and improvements will be added throughout the semester, including assignment templates, annotated solutions, and optimization exercises (constant folding, better error messages, pretty printer, etc.). The course is taught by Dr. Qi Li. You can see his course materials and assignment list at: https://github.com/qilimk/CSCI282L-2026SPRING for official instructions and additional resources.

