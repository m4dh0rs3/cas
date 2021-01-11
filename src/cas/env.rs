use {
    super::{
        expr::{atom::symbol::Symbol, Expr, TypeErr},
        parser::ParserErr,
    },
    std::{collections::HashMap, fs, io, io::BufRead},
};

pub struct Env(pub(crate) HashMap<Symbol, Def>);

#[derive(Clone)]
pub enum Def {
    Expr(Expr),
    Call { args: Vec<Symbol>, call: Expr },
    OSCall,
}

impl Env {
    pub fn new() -> Env {
        Env(HashMap::new())
    }

    pub fn get(&self, symbol: &Symbol) -> Result<&Def, TypeErr> {
        self.0
            .get(symbol)
            .ok_or_else(|| TypeErr(format!("`{}` is undefined", symbol)))
    }

    pub fn insert(&mut self, symbol: Symbol, def: Def) -> Option<Def> {
        self.0.insert(symbol, def)
    }

    pub fn remove(&mut self, symbol: Symbol) -> Option<Def> {
        self.0.remove(&symbol)
    }

    pub fn load(path: &str) -> Result<Env, String> {
        let mut env = Env::new();

        let file = fs::File::open(path).map_err(|error| format!("{:?}", error))?;
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
        Env::load("default_env.txt").unwrap_or_else(|_| Env::new())
    }
}
