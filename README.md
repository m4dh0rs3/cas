# `CAS` – [*Computer-Algebra-System*](https://en.wikipedia.org/wiki/Computer_algebra_system) :leftwards_arrow_with_hook:

> A **`CAS`** is an advanced symbolic calculator. It can *evaluate,
simplify, differentiate, integrate and solve* algebraic expressions. This is a crate written in [Rust](https://www.rust-lang.org/).

### Features

**Warning: This is not yet feature-complete. See [libqalculate](https://github.com/Qalculate/libqalculate) for a better alternative.**

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

### REPL

This crate comes with an interactive comand-line-interface: Read-Eval-Print-Loop.

### Built-in functions and constants

(for a full complete list see [`default_env.txt`](https://github.com/m4dh0rs3/cas/blob/master/default_env.txt))

- `π` (pi), `τ` (tau), `e` (eurler's number), `inf` (infinity), `nan` (not a number)
- trigonometric functions: `sin`, `cos`, `tan`, their inverse and hyperbole
- `abs`, `ceil`, `floor`, `trunc`, `fract`

### How to build see [Cargo for Rust](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html)

## Technical

### Algebraic expressions

`4.3 - π^2 = sin y` is an algebraic expression. It consists of atoms (`4.3, π, 2, y`) and operators (`-, ^, =, sin`). Operators manipulate atoms. We normally white such expressions in *infix-notaion*, where one has to know the precedence and associativity of an operator. The exponent (`^`) must be evaluated before the the substraction (`-`), evon though it appears later. To evaluate the expression linearly, it has to be converted into polish noation, which is basicly a single function call of function calls (`= - 4.3 ^ π 2 sin y`).

### Lexer and parser

To evaluate the string of an expression, it undergoes 3 steps:

- *Lexer* the string is split up into tokens (`4.3, -, π, ^, 2, =, sin, y`)
- *Parser* the stream of tokens will be ordered into polish notation
- *Eval* the expression tree can now be evaluated recursively
