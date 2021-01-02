# `CAS`

This is a `CAS` (=*Computer-Algebra-System*) written in Rust. It is basicaly a expandable calculator.
You can enter expressions in the console and recive an evaluation. It provides error messanges of
invalid expressions. You can define variables and fuctions. It implements trigonometric and other
helpful functions.

### Goals

- [x] Lex algebraic expressions into a token-stream
- [x] Parse token stream into a expression tree (cons list)
- [x] Evaluate expression into a number
- [ ] Simplify expressions
- [ ] Expand expressions
- [ ] Differentiate expressions
