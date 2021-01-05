# `CAS` – [*Computer-Algebra-System*](https://en.wikipedia.org/wiki/Computer_algebra_system) :leftwards_arrow_with_hook:

> A **`CAS`** is an advanced symbolic calculator. It can *evaluate,
simplify, differentiate, integrate and solve* algebraic expressions. This is a crate written in [Rust](https://www.rust-lang.org/).

### Features

**Warning: This crate is not yet feature-complete. See [libqalculate](https://github.com/Qalculate/libqalculate) for a better alternative.**

- [X] REPL interface
- [X] Lex operators, functions, symbols, deciaml numbers
- [X] Parse expressions of infix notation into a expressions tree
- [X] Define variables and functions
- [X] Simplify expressions
- [X] Evaluate expressions to single numbers
- [X] Comprehensive notation-error messages
- [ ] Arbitrary-precision arithmetic
- [ ] Integrate, differentiate expressions
- [ ] Solve single- and multi-variable equations / inequalities

### How to build see [Cargo for Rust](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html)

### REPL

This crate comes with an interactive comand-line-interface: Read-Eval-Print-Loop.

### Built-in functions and constants

(for a full complete list see [`default_env.txt`](https://github.com/m4dh0rs3/cas/blob/master/default_env.txt))

- `π` (pi), `τ` (tau), `e` (eurler's number), `inf` (infinity), `nan` (not a number)
- trigonometric functions: `sin`, `cos`, `tan`, their inverse and hyperbole
- `abs`, `ceil`, `floor`, `trunc`, `fract`