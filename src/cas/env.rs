use {
    super::{
        expr::{atom::symbol::Symbol, Expr, TypeErr},
        parser::ParserErr,
    },
    std::{collections::HashMap, fs, io, io::BufRead},
};

/// Contains all variables and functions as definitions.
/// Use Default to initiate with common definitions.
pub struct Env(pub(crate) HashMap<Symbol, Def>);

/// Anything denoted by symbol := value will be a definition
#[derive(Clone)]
pub enum Def {
    // Can also be another definition or even a symbol
    Expr(Expr),
    // Call math functions
    Call { args: Vec<Symbol>, call: Expr },
    // Call Rust functions
    OSCall,
}

impl Env {
    /// Default is recommended instead, because it contains common definitions.
    pub fn new() -> Env {
        Env(HashMap::new())
    }

    /// Return a definition by symbol.
    pub fn get(&self, symbol: &Symbol) -> Result<&Def, TypeErr> {
        self.0
            .get(symbol)
            .ok_or_else(|| TypeErr(format!("`{}` is undefined", symbol)))
    }

    /// Define new definition.
    pub fn insert(&mut self, symbol: Symbol, def: Def) -> Option<Def> {
        self.0.insert(symbol, def)
    }

    /// Remove definition by symbol.
    pub fn remove(&mut self, symbol: Symbol) -> Option<Def> {
        self.0.remove(&symbol)
    }

    /// Loads a file of definitions. The crate come with `default_env.txt`.
    /// Did work with WASM, probably because it is an internal file.
    pub fn load(path: &str) -> Result<Env, String> {
        let mut env = Env::new();

        let file = fs::File::open(path).map_err(|error| format!("{:?}", error))?;
        for input in io::BufReader::new(file).lines() {
            let expr = match Expr::parse(&input.map_err(|error| format!("{:?}", error))?, &env) {
                Ok(expr) => expr,
                Err(ParserErr::Empty) => continue,
                Err(error) => return Err(format!("{}", error)),
            };

            // To save a definition, eval it as an expression in its enviroment
            expr.eval(&mut env).map_err(|error| format!("{}", error))?;
        }

        Ok(env)
    }
}

impl Default for Env {
    /// Recomended initiation-point.
    fn default() -> Self {
        Env::load("default_env.txt").unwrap_or_else(|_| Env::new())
    }
}
