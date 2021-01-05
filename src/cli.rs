use std::{io, io::Write};

use super::cas::env::Env;
use super::cas::expr::Expr;

mod display;

pub struct REPL {
    ans: Option<Expr>,
    env: Env,
}

impl REPL {
    pub fn start() {
        let mut repl = REPL {
            ans: None,
            env: Env::default(),
        };

        println!("\n\u{1b}[31m\u{1b}[1mCAS\u{1b}[0m `Ctrl+C` to quit");

        loop {
            let mut input = String::new();

            print!("\u{1b}[1m\u{1b}[34m|>\u{1b}[0m ");
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Err(error) => {
                    println!(
                        "\u{1b}[31m\u{1b}[1mError:\u{1b}[0m Failed to read line: {:?}",
                        error
                    );
                    continue;
                }
                _ => {}
            }

            let expr = match Expr::parse(&input, &repl.env) {
                Ok(expr) => expr,
                Err(error) => {
                    println!("\u{1b}[31m\u{1b}[1mError:\u{1b}[0m {}", error);
                    continue;
                }
            };

            let result = match expr.eval(&mut repl.env) {
                Ok(expr) => format!("{}", expr),
                Err(error) => format!("{}", error),
            };

            let number = match expr.number(&mut repl.env) {
                Ok(number) => format!("{}", number),
                Err(error) => format!("{}", error),
            };

            println!("{} =? {} = {}", expr, result, number);
        }
    }
}
