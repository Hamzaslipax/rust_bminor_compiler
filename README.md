_# M203 Domain Specific Languages

# B-Minor Compiler

## Projektübersicht

Dieses Projekt ist ein Compiler für die Programmiersprache B-Minor, der Quellcode in B-Minor (.b) in Assembler-Code (.s) für x86_64 Linux-Systeme übersetzt. Der generierte Assembler-Code kann mit der GCC-Toolchain zu einem ausführbaren Programm assembliert und gelinkt werden.

## Projektstruktur

Das Projekt besteht aus mehreren Schritten, die alle innerhalb eines Prozesses durchgeführt werden. Die Zwischenformate (AST und IR) sind "in memory" und können für Debugging-Zwecke lesbar ausgegeben werden.

### Verzeichnisse und Dateien

- `frontend/`
    - `lexer.rs`: Implementierung des Lexers
    - `parser.rs`: Implementierung des Parsers
    - `parser.lalrpop`: Implementierung der Expressions
    - `ast.rs`: Definition des abstrakten Syntaxbaums (AST)


- `middle/`
    - `semantic_analysis.rs`: Semantische Analyse und Typsystem
    - `ir.rs`: Generierung der Intermediate Representation (IR)

- `backend/`
    - `x86_assembler_generator.rs`: Generierung des Assembler-Codes für x86_64
    - `asem.asm`: Generierte Assembler Code

- `main.rs`: Enthält testing Fälle für die Assembler Code

- `Cargo.toml`: Konfigurationsdatei für das Rust-Projekt


### Voraussetzungen

- Rust (aktuelle Version)
- GCC
- NASM (Netwide Assembler)

### Installation

1. Klone das Repository:
   ```sh
   git clone https://gitlab.rz.htw-berlin.de/s0567004/m203-domain-specific-languages.git
   cd /home/amine/m203-domain-specific-languages
   Build the project with: cargo run
   Open another Terminal
   cd /home/amine/m203-domain-specific-languages/backend/src
   run: ./output

### How to use

cargo run -- -i <input-file> -o <output-file> [-v]
or
./target/debug/binary_name -i <input-file> -o <output-file> [-v]
