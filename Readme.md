# Rcal::Rust CLI Arithmetic Expression Evaluator

Rcal is a **command-line arithmetic expression evaluator written in Rust**.  
It parses and evaluates mathematical expressions provided as free-form text, enforcing correct **operator precedence**,
**associativity**, and **parentheses handling**, while providing robust validation and clear error messages.

Unlike simple calculators, Rcal follows a **compiler-style pipeline**: tokenization → parsing → evaluation.

---

## Features

- **Free-form input**  
  Expressions may include or omit spaces:  
  `2*(3+4.5)` and `2 * ( 3 + 4.5 )` are both valid.

- **Floating-point number support**  
  Handles decimal numbers with proper validation.

- **Operator precedence & associativity**  
  Correctly evaluates expressions involving:
    - Addition (`+`)
    - Subtraction (`-`)
    - Multiplication (`*`)
    - Division (`/`)
    - Exponentiation (`^`)
    - Unary negation (`-x`)

- **Parentheses handling**  
  Nested and grouped expressions are evaluated with higher precedence.

- **Robust error handling**  
  Gracefully aborts with descriptive errors for:
    - Unsupported characters or variables
    - Unrecognized operators
    - Malformed numbers
    - Mismatched parentheses
    - Invalid expression structure

- **Safe evaluation**  
  No use of `eval` or dynamic execution — all parsing and computation are explicit and deterministic.

---

## Architecture

Rcal is structured as a three-stage evaluation pipeline:

### Tokenization (Lexical Analysis)

The input string is scanned character-by-character and converted into tokens:

- Numbers
- Operators
- Parentheses

Invalid characters and malformed numbers are detected at this stage.

### Parsing

The token stream is transformed into AST.
This ensures:

- Correct operator precedence
- Proper handling of parentheses
- Support for unary operators

### Evaluation

The expression is evaluated, producing a single numeric result.

---

## Usage

### Run from CLI

```bash
cargo run
``` Input your expression and press Enter.
