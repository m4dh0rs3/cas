use crate::cas::{lexer::*, Env};

#[test]
fn general() {
    let env = Env::new();
    let mut lexer = Lexer::new("3.2e-1+2^.1*2abc==", &env);

    assert_eq!(lexer.advance(), Ok(Token::Atom(Atom::Number(Number(0.32)))));
    assert_eq!(lexer.advance(), Ok(Token::Op(Op::Add)));
    assert_eq!(lexer.advance(), Ok(Token::Atom(Atom::Number(Number(2.0)))));
    assert_eq!(lexer.advance(), Ok(Token::Op(Op::Exp)));
    assert_eq!(lexer.advance(), Ok(Token::Atom(Atom::Number(Number(0.1)))));
    assert_eq!(lexer.advance(), Ok(Token::Op(Op::Mul)));
    assert_eq!(lexer.advance(), Ok(Token::Atom(Atom::Number(Number(2.0)))));
    assert_eq!(
        lexer.advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("abc")))))
    );
    assert_eq!(lexer.advance(), Ok(Token::Op(Op::Eq)));
    assert_eq!(lexer.advance(), Err(LexerErr::EOF));

    let mut lexer = Lexer::new("(x,y)", &env);

    assert_eq!(lexer.advance(), Ok(Token::Op(Op::Open)));
    assert_eq!(
        lexer.advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("x")))))
    );
    assert_eq!(lexer.advance(), Ok(Token::Op(Op::List)));
    assert_eq!(
        lexer.advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("y")))))
    );
    assert_eq!(lexer.advance(), Ok(Token::Op(Op::Close)));
    assert_eq!(lexer.advance(), Err(LexerErr::EOF));
}

#[test]
fn combinations() {
    let env = Env::new();

    assert_eq!(
        Lexer::new("3x", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(3.0))))
    );
    assert_eq!(
        Lexer::new("4x", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(4.0))))
    );
    assert_eq!(
        Lexer::new("0.12x", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(0.12))))
    );
    assert_eq!(
        Lexer::new("0.12e2x", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(12.0))))
    );
}

#[test]
fn symbols() {
    let env = Env::new();

    assert_eq!(
        Lexer::new("test   ", &env).advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("test")))))
    );
    assert_eq!(
        Lexer::new("TEST", &env).advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("TEST")))))
    );
    assert_eq!(
        Lexer::new("teSt  ", &env).advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("teSt")))))
    );
    assert_eq!(
        Lexer::new("    teSt", &env).advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("teSt")))))
    );
    assert_eq!(
        Lexer::new("    te    St", &env).advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("te")))))
    );
    assert_eq!(
        Lexer::new("te    St", &env).advance(),
        Ok(Token::Atom(Atom::Symbol(Symbol(String::from("te")))))
    );
}

#[test]
fn numbers() {
    let env = Env::new();

    assert_eq!(
        Lexer::new("0.00012", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(12.0e-5))))
    );
    assert_eq!(
        Lexer::new("1234.56789", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(1234.56789))))
    );
    assert_eq!(
        Lexer::new("0000.56789", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(0.56789))))
    );
    assert_eq!(
        Lexer::new(".1", &env).advance(),
        Ok(Token::Atom(Atom::Number(Number(0.1))))
    );
}

#[test]
fn operators() {
    let env = Env::new();

    assert_eq!(Lexer::new("+", &env).advance(), Ok(Token::Op(Op::Add)));
    assert_eq!(Lexer::new("-", &env).advance(), Ok(Token::Op(Op::Sub)));
    assert_eq!(Lexer::new("==", &env).advance(), Ok(Token::Op(Op::Eq)));
    assert_eq!(Lexer::new(",", &env).advance(), Ok(Token::Op(Op::List)));
    assert_eq!(Lexer::new("(", &env).advance(), Ok(Token::Op(Op::Open)));
    assert_eq!(Lexer::new("[", &env).advance(), Ok(Token::Op(Op::Open)));
    assert_eq!(Lexer::new("{", &env).advance(), Ok(Token::Op(Op::Open)));
    assert_eq!(Lexer::new("]", &env).advance(), Ok(Token::Op(Op::Close)));
    assert_eq!(Lexer::new("}", &env).advance(), Ok(Token::Op(Op::Close)));

    assert_eq!(Lexer::new("====", &env).advance(), Ok(Token::Op(Op::Eq)));
    assert_eq!(Lexer::new("-=", &env).advance(), Ok(Token::Op(Op::Sub)));
    assert_eq!(Lexer::new("!=", &env).advance(), Ok(Token::Op(Op::Neq)));
    assert_eq!(Lexer::new("~=", &env).advance(), Ok(Token::Op(Op::Neq)));
}

#[test]
fn expressions() {
    let env = Env::new();
    //println!("{:#?}", Lexer::new("3x^2", &env).collect::<Vec<Token>>());
}

// TODO: cast Result<Token, LexerErr> to Token { Token::Default.., LexerErr }
// No, cause ? Op would not be accesable...
