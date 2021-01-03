//! # `CAS` – Computer Algebra System
//! A `CAS` evaluates, simplifies and changes *algebraic expressions*
//! like `2.3e-3*x^2=4/n!`.
//! These expressions are split up into `Token`s (`Atom`s and `Op`erators) by the `Lexer`.
//! The `Parser` rearanges them into *S–Expressions*, also known as *Cons–Lists* of the
//! form `Expr = { Op ~ Expr }`

use lexer::Op;

use self::{
    lexer::{Atom, Number, Symbol},
    parser::{Call, Expr},
};
use std::{collections::HashMap, io, io::Write};

pub(crate) mod lexer;
pub(crate) mod parser;

/// Saves historic expressions, like `x:=5`, so `x` evals to `5`.
/// Also holds function definitions and constants as default, e.g. PI, cos()
pub struct Env {
    defs: HashMap<Symbol, Def>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            defs: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub(crate) enum Def {
    Call(Vec<Symbol>, Box<Expr>),
    OSCall,
    Atom(Expr),
}

impl Env {
    pub fn run(&mut self) {
        println!("\n\u{1b}[31m\u{1b}[1mCAS\u{1b}[0m `Ctrl+C` to quit");

        loop {
            let mut input = String::new();

            print!("\u{1b}[1m\u{1b}[34m|>\u{1b}[0m ");
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Err(e) => {
                    println!("\u{1b}[31m\u{1b}[1mError:\u{1b}[0m Failed to read line");
                    continue;
                }
                _ => {}
            }

            let expr = match Expr::new(&input, self) {
                Ok(expr) => expr,
                Err(e) => {
                    println!("\u{1b}[31m\u{1b}[1mError:\u{1b}[0m {}", e);
                    continue;
                }
            };

            let result = match expr.clone().number(self) {
                Ok(number) => format!("{}", number),
                Err(error) => format!("{}", error),
            };

            println!("{} ?= {}", expr, result);
        }
    }

    pub(crate) fn update(&mut self, env: Env) {
        for (symbol, def) in env.defs.into_iter() {
            self.defs.insert(symbol, def);
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        let mut env = Env::new();

        env.defs.insert(
            Symbol(String::from("true")),
            Def::Atom(Expr::Atom(Atom::Number(Number(1.0)))),
        );
        env.defs.insert(
            Symbol(String::from("false")),
            Def::Atom(Expr::Atom(Atom::Number(Number(0.0)))),
        );

        env.defs.insert(
            Symbol(String::from("pi")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::consts::PI)))),
        );
        env.defs.insert(
            Symbol(String::from("π")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::consts::PI)))),
        );

        env.defs.insert(
            Symbol(String::from("tau")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::consts::TAU)))),
        );
        env.defs.insert(
            Symbol(String::from("τ")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::consts::TAU)))),
        );

        env.defs.insert(
            Symbol(String::from("e")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::consts::E)))),
        );

        env.defs.insert(
            Symbol(String::from("i")),
            Def::Atom(Expr::Call(Call::new(
                Op::Call(Symbol(String::from("sqrt"))),
                vec![Expr::Atom(Atom::Number(Number(-1.0)))],
            ))),
        );

        env.defs.insert(
            Symbol(String::from("inf")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::INFINITY)))),
        );

        env.defs.insert(
            Symbol(String::from("NaN")),
            Def::Atom(Expr::Atom(Atom::Number(Number(std::f64::NAN)))),
        );

        env.defs.insert(Symbol(String::from("sin")), Def::OSCall);
        env.defs.insert(Symbol(String::from("asin")), Def::OSCall);
        env.defs.insert(Symbol(String::from("sinh")), Def::OSCall);
        env.defs.insert(Symbol(String::from("asinh")), Def::OSCall);

        env.defs.insert(Symbol(String::from("cos")), Def::OSCall);
        env.defs.insert(Symbol(String::from("acos")), Def::OSCall);
        env.defs.insert(Symbol(String::from("cosh")), Def::OSCall);
        env.defs.insert(Symbol(String::from("acosh")), Def::OSCall);

        env.defs.insert(Symbol(String::from("tan")), Def::OSCall);
        env.defs.insert(Symbol(String::from("atan")), Def::OSCall);
        env.defs.insert(Symbol(String::from("tanh")), Def::OSCall);
        env.defs.insert(Symbol(String::from("atanh")), Def::OSCall);

        env.defs.insert(Symbol(String::from("log")), Def::OSCall);
        env.defs.insert(Symbol(String::from("ln")), Def::OSCall);

        env.defs.insert(Symbol(String::from("sqrt")), Def::OSCall);
        env.defs.insert(Symbol(String::from("root")), Def::OSCall);

        env.defs.insert(Symbol(String::from("mod")), Def::OSCall);
        env.defs.insert(Symbol(String::from("abs")), Def::OSCall);
        env.defs.insert(Symbol(String::from("gcd")), Def::OSCall);
        env.defs.insert(Symbol(String::from("lcm")), Def::OSCall);

        env.defs.insert(Symbol(String::from("diff")), Def::OSCall);
        env.defs.insert(Symbol(String::from("itgr")), Def::OSCall);

        env.defs.insert(Symbol(String::from("sum")), Def::OSCall);

        env
    }
}
