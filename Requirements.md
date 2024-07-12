# Compiler Project Requirements (DRAFT)

## General Information
- **Programming Languages:**
  - Frontend: Rust
  - Middle and Backend: C

## Supported Input & Output
- **Input:** B-Minor source code files (.b)
- **Output:** Assembly code (.s) for x86_64 Linux systems

## Intermediate Representations
- Abstract Syntax Tree (AST)
- Intermediate Representation (IR)

## Frontend (Rust)
1. **Lexer (Scanner):**
   - Implement lexer using Logos.
   - Define token patterns for the following:
     - Keywords: `int`, `if`, `else`, `while`, `return`, `break`, `continue`
     - Identifiers
     - Operators: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`
     - Delimiters: `(`, `)`, `{`, `}`, `,`, `;`
     - Literals: integers, strings
     - Comments: single-line (`//`) and multi-line (`/* */`)
   - Provide error handling for invalid tokens.

2. **Parser:**
   - Implement parser using Lalrpop.
   - Create grammar rules to generate AST for variable declarations, functions, expressions, statements, and program structure.
   - Report syntax errors with clear messages.

3. **AST Generation:**
   - Design suitable data structures for AST nodes.
   - Construct AST during parsing.
   - Include helper data structures like Symbol Table and Type System.

## Middle-End (C)
1. **Semantic Analysis:**
   - Implement semantic analysis on AST for symbol resolution, type compatibility, and function checking.

2. **AST Optimization:**
   - Perform simple optimizations like constant folding and dead code elimination on AST.

3. **IR Generation:**
   - Define IR structure.
   - Generate IR from AST.

## Backend (C)
1. **Architecture Mapping:**
   - Map IR to target architecture (x86_64).
   - Implement register assignment, stack layout, and ABI compliance.

2. **Optimization:**
   - Implement basic optimizations like constant propagation and common subexpression elimination.

3. **Assembly Code Generation:**
   - Translate IR into x86_64 Assembly code (.s).
   - Produce human-readable assembly file with comments.

## Additional Requirements
- **Debugging Outputs:**
  - Provide options for outputting intermediate representations.
- **Testing and Automation:**
  - Create comprehensive unit tests and regression tests.
- **Documentation:**
  - Include detailed project documentation in README.md.
- **Code Quality:**
  - Adhere to consistent code style and write meaningful comments.

## Why Rust and C?
- **Rust (Frontend):**
  - Rust provides strong memory safety guarantees, preventing common programming errors like null pointer dereferencing and buffer overflows.
  - Rust's modern language features and Storage Safety makes it well suited for implementing the frontend of the compiler, where complex parsing and analysis tasks are performed.

- **C (Middle and Backend):**
  - C is a low-level language with direct access to memory and hardware, making it ideal for implementing the middle and backend of the compiler.
  - Its simplicity and efficiency allow for fine-grained control over resource usage and performance optimization.
  - Also C has a long history of being used in compiler development.
