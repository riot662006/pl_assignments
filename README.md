# Programming Languages Assignment Collection

This repository collects my Programming Languages compiler assignments for Spring 2026. Each folder builds a slightly larger version of the same S-expression based language, moving from simple arithmetic to variables, then to booleans, control flow, mutation, and runtime type checking.

The projects are written in Rust and generate x86-64 assembly, which is then assembled and linked with a small Rust runtime.

## Repository Layout

- `cobra/` - Week 3 compiler with booleans, conditionals, loops, mutation, and tagged runtime values
- `boa/` - Week 2 compiler with variables, `let` bindings, and binary arithmetic
- `adder/` - Week 1 compiler with numeric literals and unary arithmetic
- `_starter_code/` - starter template provided for the assignment sequence

## Cobra (Week 3)

- **Title:** Cobra - Booleans, Conditionals, and Loops
- **Due:** Fri Mar 27, 2026
- **Overview:** Extends the compiler to support booleans, conditionals, loops, mutation with `set!`, blocks, comparisons, and runtime type checking.

**What is implemented**

- Tagged runtime values for numbers and booleans
- Boolean literals: `true`, `false`
- Unary operations: `add1`, `sub1`, `negate`, `isnum`, `isbool`
- Binary operations: `+`, `-`, `*`, `<`, `>`, `<=`, `>=`, `=`
- Variables and `let` bindings
- Mutation with `set!`
- Multi-expression `block`
- Control flow with `if`, `loop`, and `break`
- Runtime invalid-argument errors through `snek_error`
- Compiler-side checks for invalid identifiers, duplicate bindings, unbound variables, invalid blocks, and `break` outside loops

**Current structure**

- `cobra/src/main.rs` - parser, AST, tagged-value code generation, label management, and compiler error handling
- `cobra/runtime/start.rs` - runtime entry point plus printing of tagged booleans and numbers
- `cobra/test/legacy_tests/` - carried-forward tests from earlier assignments
- `cobra/test/error_cases/` - programs that should fail
- `cobra/test/my_test_cases/` - additional custom Cobra test programs
- `cobra/Makefile` - build, run, transcript, and clean targets

## Boa (Week 2)

- **Title:** Boa - Variables and Binary Operators
- **Due:** Fri Mar 13, 2026
- **Overview:** Extends the compiler to support variables, `let` bindings, stack allocation, and binary arithmetic operators.

**What is implemented**

- Numeric literals and identifiers
- Unary operations: `add1`, `sub1`, `negate`
- Binary operations: `+`, `-`, `*`
- `let` bindings with stack-based variable storage
- Environment tracking with a Rust `HashMap`
- Shadowing support through nested environments
- Compiler-side checks for duplicate bindings, keyword misuse, and unbound variables

**Current structure**

- `boa/src/main.rs` - AST definitions, parser, environment-based code generation, and assembly emission
- `boa/runtime/start.rs` - runtime entry point for printing numeric results
- `boa/test/` - provided Boa tests
- `boa/test/error_cases/` - invalid programs
- `boa/test/my_test_cases/` - extra custom tests
- `boa/Makefile` - build, run, transcript, and clean targets

## Adder (Week 1)

- **Title:** Adder
- **Due:** Fri Feb 20, 2026
- **Overview:** Builds the first minimal compiler in the sequence, covering parsing, AST construction, unary arithmetic, and assembly generation.

**What is implemented**

- Numeric literals
- Unary operations: `add1`, `sub1`, `negate`
- Parsing from S-expressions into a Rust AST
- Direct x86-64 code generation into `rax`
- Assembly emission and linking with a small runtime

**Current structure**

- `adder/src/main.rs` - parser, AST, and code generator for the Week 1 language
- `adder/runtime/start.rs` - runtime entry point
- `adder/test/` - example programs and transcripts
- `adder/Makefile` - build and clean targets

## Running the Projects

Each assignment folder is self-contained. Run commands from inside the relevant folder.

```bash
cd cobra
make test
```

You can also generate a transcript for a specific test folder:

```bash
cd cobra
make transcript DIR=test/my_test_cases
```

The same pattern works for `boa/` and `adder/`, using their local `Makefile`s and test directories.

## Notes

- All three compilers read a `.snek` input file and produce `.s` assembly output.
- The generated assembly is assembled with NASM and linked with the runtime using `ar` and `rustc`.
- Later assignments build directly on ideas introduced in earlier ones, so the repo is organized to show that progression clearly.

Course materials and official assignment instructions were provided by Dr. Qi Li.
