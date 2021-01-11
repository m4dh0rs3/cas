pub mod cas;
mod cli;

pub mod prelude {
    pub use crate::{
        cas::{
            env::{Def, Env},
            expr::{
                atom::{number::Number, symbol::Symbol, Atom},
                TypeErr,
            },
        },
        cli::REPL,
    };
}

#[cfg(test)]
mod tests;
