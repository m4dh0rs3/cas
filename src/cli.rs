use {
    super::cas::{
        env::{Def, Env},
        expr::{
            atom::{symbol::Symbol, Atom},
            Expr,
        },
    },
    std::{io, io::Write},
};

mod display;

/// This crate comes with an interactive comand-line-interface: Read-Eval-Print-Loop.
/// It contains an enviroment.
pub struct REPL {
    env: Env,
}

impl REPL {
    /// Execute this in main to start the REPL.
    pub fn start() {
        let mut repl = REPL {
            // contains basic variables and functions
            env: Env::default(),
        };

        // color-code formats
        // `Ctrl+C` is a keyboard interupt to abort the programm
        println!("\n\u{1b}[31;1mCAS\u{1b}[0m `Ctrl+C` to quit");

        loop {
            let mut input = String::new();

            print!("\u{1b}[93;1m|>\u{1b}[0m ");
            // make shure its printed by flushing to stdout
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Err(error) => {
                    println!(
                        "\u{1b}[31;1mError:\u{1b}[0m Failed to read line: {:?}\n",
                        error
                    );
                    // restart the loop if the input could not be handled
                    continue;
                }
                _ => {}
            }

            // parsing the expression already needs all defined symbols and functions
            // to decide if there is a function call or a multiplication
            let expr = match Expr::parse(&input, &repl.env) {
                Ok(expr) => expr,
                Err(error) => {
                    println!("\u{1b}[31;1mError:\u{1b}[0m {}\n", error);
                    continue;
                }
            };

            println!("\u{1b}[94;1m|:\u{1b}[0m {}", &expr);

            let result = match expr.eval(&mut repl.env) {
                Ok(expr) => expr,
                Err(error) => {
                    println!("\u{1b}[31;1mError:\u{1b}[0m {}\n", error);
                    continue;
                }
            };

            // try to eval to a number
            match result.number(&mut repl.env) {
                Ok(number) => println!("\u{1b}[91;1m|≈\u{1b}[0m {}", number),
                Err(_) => { /* println!("\u{1b}[31;1m|=\u{1b}[0m {}", &result) */ }
            }

            // free line
            println!();

            // implements ans to copy a result
            if &result != &Expr::Atom(Atom::Symbol(Symbol(String::from("ans")))) {
                repl.env
                    .insert(Symbol("ans".to_string()), Def::Expr(result));
            }
        }
    }
}
