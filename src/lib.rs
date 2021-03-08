//! # `Taschenrechner` – [*Computer-Algebra-System*](https://en.wikipedia.org/wiki/Computer_algebra_system)
//!
//! > A **`CAS`** is an advanced symbolic calculator. It can *evaluate,
//! simplify, differentiate, integrate and solve* algebraic expressions.
//!
//! ```
//! use cas::prelude::*;
//!
//! fn main() {
//!     REPL::start();
//! }
//! ```
//! ### Features
//!
//! **Warning: This is not yet feature-complete. See [libqalculate](https://github.com/Qalculate/libqalculate) for a better alternative.**
//!
//! - REPL interface
//! - Lex operators, functions, symbols, deciaml numbers
//! - Parse expressions of infix notation into a expressions tree
//! - Define variables and functions
//! - Simplify expressions
//! - Evaluate expressions to single numbers
//! - Comprehensive notation-error messages
//!
//! ### TODO
//!
//! - Arbitrary-precision arithmetic
//! - Integrate, differentiate expressions
//! - Solve single- and multi-variable equations / inequalities
//!
//! #### Eval
//!
//! The current eval is not very sophisticated. It is functional, but can't
//! simplify any expressions, if a subexpression remains unresolved.
//!
//! ### REPL
//!
//! This crate comes with an interactive comand-line-interface: Read-Eval-Print-Loop.
//!
//! ### Built-in functions and constants
//!
//! (for a full complete list see [`default_env.txt`](https://github.com/m4dh0rs3/cas/blob/master/default_env.txt))
//!
//! - `π` (pi), `τ` (tau), `e` (eurler's number), `inf` (infinity), `nan` (not a number)
//! - trigonometric functions: `sin`, `cos`, `tan`, their inverse and hyperbole
//! - `abs`, `ceil`, `floor`, `trunc`, `fract`
//!
//! ### How to build see [Cargo for Rust](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html)
//!
//! ## Technical
//!
//! > Also see my [blog post](https://m4dh0rs3.github.io/posts/cas/)!
//!
//! ### Algebraic expressions
//!
//! `4.3 - π^2 = sin y` is an algebraic expression. It consists of atoms (`4.3, π, 2, y`) and operators (`-, ^, =, sin`). Operators manipulate atoms. We normally write such expressions in *infix-notaion*, where one has to know the precedence and associativity of an operator. The exponent (`^`) must be evaluated before the the substraction (`-`), evon though it appears later. To evaluate the expression linearly, it has to be converted into polish noation, which is basicly a single function call of function calls (`= - 4.3 ^ π 2 sin y`).
//!
//! ### Lexer and parser
//!
//! To evaluate the string of an expression, it undergoes 3 steps:
//!
//! - *Lexer* the string is split up into tokens (`4.3, -, π, ^, 2, =, sin, y`)
//! - *Parser* the stream of tokens will be ordered into polish notation
//! - *Eval* the expression tree can now be evaluated recursively
mod cas;
mod cli;

/// The prelude module contains all the types to get you started.
pub mod prelude {
    pub use crate::{
        cas::{
            env::{Def, Env},
            expr::{
                atom::{number::Number, symbol::Symbol, Atom},
                Expr, TypeErr,
            },
        },
        cli::REPL,
    };
}

#[cfg(test)]
mod tests;
