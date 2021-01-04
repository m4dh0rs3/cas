use super::expr::Expr;
use super::expr::TypeErr;
use super::expr::{atom::symbol::Symbol, call::Call};
use std::collections::HashMap;

pub(crate) struct Env(pub(crate) HashMap<Symbol, Def>);

#[derive(Clone)]
pub(crate) enum Def {
    Expr(Expr),
    Call { args: Vec<Symbol>, call: Call },
    OSCall,
}

impl Env {
    pub(crate) fn get(&self, symbol: &Symbol) -> Result<&Def, TypeErr> {
        self.0
            .get(symbol)
            .ok_or_else(|| TypeErr(format!("`{}` is undefined", symbol)))
    }
}
