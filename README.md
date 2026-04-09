# Programming Languages Assignment Collection

This repository collects my Spring 2026 Programming Languages compiler assignments. Each project extends the same S-expression-based language a little further, moving from simple arithmetic to variables, booleans, control flow, mutation, runtime checks, and first-order functions.

The compilers are written in Rust and emit x86-64 assembly, which is then assembled and linked with a small Rust runtime.

## Repository Layout

- `adder/` - Week 1 compiler with numeric literals and unary arithmetic
- `boa/` - Week 2 compiler with variables, `let` bindings, and binary arithmetic
- `cobra/` - Week 3 compiler with booleans, conditionals, loops, mutation, and tagged values
- `diamondback/` - Week 4 compiler with top-level function definitions and function calls
- `_starter_code/` - starter template provided for the assignment sequence

## Diamondback (Week 4)

- **Title:** Diamondback - Functions and Calling Conventions
- **Overview:** Extends Cobra with top-level function definitions, function calls, and argument passing.

**What is implemented**

- Top-level function definitions with `(fun (<name> <arg>*) <expr>)`
- Function calls with zero or more arguments
- Tagged runtime values for numbers and booleans
- Unary operations: `add1`, `sub1`, `negate`, `isnum`, `isbool`
- Binary operations: `+`, `-`, `*`, `<`, `>`, `<=`, `>=`, `=`
- Variables and `let` bindings
- Mutation with `set!`
- Multi-expression `block`
- Control flow with `if`, `loop`, and `break`
- Runtime invalid-argument errors through `snek_error`
- Stack-frame based local-variable storage and call-site stack alignment for function calls

**Current structure**

- `diamondback/src/lib.rs` - parser, AST, function-aware code generation, stack-frame management, and compiler checks
- `diamondback/src/main.rs` - compiler entry point plus optional `--debug` assembly dump mode
- `diamondback/runtime/start.rs` - runtime entry point plus printing for tagged booleans and numbers
- `diamondback/test/` - provided and custom Diamondback tests
- `diamondback/test/my_test_cases/` - extra custom test programs
- `diamondback/Makefile` - build, run, transcript, and clean targets

## Cobra (Week 3)

- **Title:** Cobra - Booleans, Conditionals, and Loops
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

## Boa (Week 2)

- **Title:** Boa - Variables and Binary Operators
- **Overview:** Extends the compiler to support variables, `let` bindings, stack allocation, and binary arithmetic operators.

**What is implemented**

- Numeric literals and identifiers
- Unary operations: `add1`, `sub1`, `negate`
- Binary operations: `+`, `-`, `*`
- `let` bindings with stack-based variable storage
- Environment tracking with a Rust `HashMap`

## Adder (Week 1)

- **Title:** Adder
- **Overview:** Builds the first minimal compiler in the sequence, covering parsing, AST construction, unary arithmetic, and assembly generation.

**What is implemented**

- Numeric literals
- Unary operations: `add1`, `sub1`, `negate`
- Parsing from S-expressions into a Rust AST
- Direct x86-64 code generation into `rax`

## Running the Projects

Each assignment folder is self-contained. Run commands from inside the relevant folder.

Example:

```bash
cd diamondback
make test
```

To run a specific directory of tests:

```bash
cd diamondback
make test DIR=test/my_test_cases
```

To generate a transcript for a directory:

```bash
cd diamondback
make transcript DIR=test/my_test_cases
```

For a single program:

```bash
cd diamondback
make test/my_test_cases/011.run
./test/my_test_cases/011.run
```

To inspect parsing and generated assembly without writing an output file:

```bash
cd diamondback
cargo run -- --debug test/my_test_cases/011.snek
```

The same `make test`, `make transcript`, and `make clean` pattern also applies to the earlier assignment folders where supported.

## Notes

- All compilers read `.snek` input files and produce `.s` assembly output.
- The generated assembly is assembled with NASM and linked with the runtime using `ar` and `rustc`.
- In `diamondback/`, the `Makefile` rebuilds assembly when any file in `src/` changes.
- In `diamondback/`, it is safest to rebuild and run one `.run` target at a time because the runtime link step reuses shared files in `runtime/`.
- Later assignments build directly on ideas introduced in earlier ones, so the repo is organized to show that progression clearly.

Course materials and official assignment instructions were provided by Dr. Qi Li.
