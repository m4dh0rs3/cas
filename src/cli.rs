use std::{io, io::Write};

use super::cas::env::Def;
use super::cas::env::Env;
use super::cas::expr::atom::{symbol::Symbol, Atom};
use super::cas::expr::Expr;

mod display;

pub struct REPL {
    env: Env,
}

impl REPL {
    pub fn start() {
        let mut repl = REPL {
            env: Env::default(),
        };

        println!("\n\u{1b}[31;1mCAS\u{1b}[0m `Ctrl+C` to quit");

        loop {
            let mut input = String::new();

            print!("\u{1b}[93;1m|>\u{1b}[0m ");
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Err(error) => {
                    println!(
                        "\u{1b}[31;1mError:\u{1b}[0m Failed to read line: {:?}",
                        error
                    );
                    continue;
                }
                _ => {}
            }

            let expr = match Expr::parse(&input, &repl.env) {
                Ok(expr) => expr,
                Err(error) => {
                    println!("\u{1b}[31;1mError:\u{1b}[0m {}", error);
                    continue;
                }
            };

            println!("\u{1b}[94;1m|:\u{1b}[0m {}", &expr);

            let result = match expr.eval(&mut repl.env) {
                Ok(expr) => expr,
                Err(error) => {
                    println!("\u{1b}[31;1mError:\u{1b}[0m {}", error);
                    continue;
                }
            };

            match result.number(&mut repl.env) {
                Ok(number) => println!("\u{1b}[91;1m|≈\u{1b}[0m {}", number),
                Err(_) => println!("\u{1b}[31;1m|=\u{1b}[0m {}", &result),
            }

            println!();

            if &result != &Expr::Atom(Atom::Symbol(Symbol(String::from("ans")))) {
                repl.env
                    .insert(Symbol("ans".to_string()), Def::Expr(result));
            }
        }
    }
}
