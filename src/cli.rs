use super::cas::env::Env;
use super::cas::expr::Expr;

mod display;

struct REPL {
    ans: Option<Expr>,
    env: Env,
}

impl REPL {
    fn start(env: &Env) {}
}
