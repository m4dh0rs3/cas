use crate::cas::{parser::Expr, Env};

#[test]
fn general() {
    let mut env = Env::new();

    println!("{}", Expr::new("α*(2.43+3.0)^Abs:=4e-2", &env).unwrap());
    println!("{}", Expr::new("1+2+3+4+5", &env).unwrap());
    println!("{}", Expr::new("1+2+3*4+5", &env).unwrap());
    println!("{}", Expr::new("(1+2)*3*(4+5)", &env).unwrap());
    println!("{}", Expr::new("1+2*3^(4+5)", &env).unwrap());
    println!("{}", Expr::new("1-2*3^(4/5)", &env).unwrap());
    println!("{}", Expr::new("1--2*3^(4/5)", &env).unwrap());
    println!("{}", Expr::new("1---2*3^(4/5)", &env).unwrap());
    println!("{}", Expr::new("a---5", &env).unwrap());

    println!("{}", Expr::new("sin 2*x", &Env::default()).unwrap());
    println!("{}", Expr::new("sin sin 2*x", &Env::default()).unwrap());
    println!("{}", Expr::new("sin 2^x", &Env::default()).unwrap());
    println!("{}", Expr::new("sin 2+x", &Env::default()).unwrap());
    println!("{}", Expr::new("sin 2-x", &Env::default()).unwrap());
    println!("{}", Expr::new("2/sin(2-x)", &Env::default()).unwrap());
    println!("{}", Expr::new("sinsinx", &Env::default()).unwrap());
    println!("{}", Expr::new("true", &Env::default()).unwrap());
    println!("{}", Expr::new("sin(3.0, e)", &Env::default()).unwrap());
    println!("{}", Expr::new("sincos(pi)*e^i", &Env::default()).unwrap());
}
