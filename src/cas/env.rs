use super::expr::{atom::number::Number, TypeErr};
use super::expr::{atom::symbol::Symbol, call::Call};
use super::expr::{atom::Atom, Expr};
use super::parser::ParserErr;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

pub(crate) struct Env(pub(crate) HashMap<Symbol, Def>);

#[derive(Clone)]
pub(crate) enum Def {
    Expr(Expr),
    Call { args: Vec<Symbol>, call: Expr },
    OSCall,
}

impl Env {
    pub(crate) fn new() -> Env {
        Env(HashMap::new())
    }

    pub(crate) fn get(&self, symbol: &Symbol) -> Result<&Def, TypeErr> {
        self.0
            .get(symbol)
            .ok_or_else(|| TypeErr(format!("`{}` is undefined", symbol)))
    }

    pub(crate) fn insert(&mut self, symbol: Symbol, def: Def) -> Option<Def> {
        self.0.insert(symbol, def)
    }

    pub(crate) fn remove(&mut self, symbol: Symbol) -> Option<Def> {
        self.0.remove(&symbol)
    }

    pub(crate) fn load(path: &str) -> Result<Env, String> {
        let mut env = Env::new();

        let mut file = fs::File::open(path).map_err(|error| format!("{:?}", error))?;
        for input in io::BufReader::new(file).lines() {
            let expr = match Expr::parse(&input.map_err(|error| format!("{:?}", error))?, &env) {
                Ok(expr) => expr,
                Err(ParserErr::Empty) => continue,
                Err(error) => return Err(format!("{}", error)),
            };

            expr.eval(&mut env).map_err(|error| format!("{}", error))?;
        }

        Ok(env)
    }
}

impl Default for Env {
    fn default() -> Self {
        Env::load("default_env.txt").unwrap()
    }
}
