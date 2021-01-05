use super::expr::{atom::number::Number, TypeErr};
use super::expr::{atom::symbol::Symbol, call::Call};
use super::expr::{atom::Atom, Expr};
use std::collections::HashMap;

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
}

impl Default for Env {
    fn default() -> Self {
        Env::new()
    }
}
