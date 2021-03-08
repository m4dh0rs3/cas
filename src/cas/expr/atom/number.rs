use super::super::super::lexer::{
    Lexer, LexerErr, BIN_DIGITS, DEC_DIGITS, E_TOKEN_LEN, HEX_DIGITS,
};

mod ops;

/// Wrapper of a f64. Maybe implement arbitrary precicion in the future.
#[derive(Clone, Copy, PartialEq)]
pub struct Number(pub(crate) f64);

impl Number {
    pub(crate) fn parse(lexer: &mut Lexer) -> Result<Number, LexerErr> {
        /* let first = lexer
            .chars
            .peek()
            .expect("Lexer.chars.peek() None, but not before")
            .1;

        if first == '0' {
            lexer.chars.next();

            match lexer.chars.peek() {
                Some((_, 'x')) => {
                    lexer.chars.next();
                    return Number::parse_hex(lexer);
                }
                Some((_, 'b')) => {
                    lexer.chars.next();
                    return Number::parse_bin(lexer);
                }
                _ => {}
            }
        } */

        Number::parse_dec(lexer)
    }

    fn parse_dec(lexer: &mut Lexer) -> Result<Number, LexerErr> {
        let mut string = String::with_capacity(E_TOKEN_LEN);
        let start = lexer.start();

        //string.push_str(&lexer.eat(|pot| "+-".contains(*pot)));
        string.push_str(&lexer.eat(is_dec_digit));

        if let Ok((_, pot)) = lexer.expect(vec!['.']) {
            string.push(pot);
            string.push_str(&lexer.eat(is_dec_digit));
        }

        if let Ok((_, pot)) = lexer.expect(vec!['e', 'E']) {
            string.push(pot);
            string.push_str(&lexer.eat(|pot| "+-".contains(*pot)));
            string.push_str(&lexer.eat(is_dec_digit));
        }

        string
            .parse::<f64>()
            .map_err(|_| LexerErr::panic(format!("could not parse decimal"), start))
            .map(|number| Number(number))
    }

    fn parse_hex(lexer: &mut Lexer) -> Result<Number, LexerErr> {
        let mut string = String::with_capacity(E_TOKEN_LEN);
        let start = lexer.start();

        string.push_str(&lexer.eat(is_hex_digit));

        if let Ok((_, pot)) = lexer.expect(vec!['.']) {
            string.push(pot);
            string.push_str(&lexer.eat(is_hex_digit));
        }

        string
            .parse::<f64>()
            .map_err(|_| LexerErr::panic(format!("could not parse hexadecimal"), start))
            .map(|number| Number(number))
    }

    fn parse_bin(lexer: &mut Lexer) -> Result<Number, LexerErr> {
        let mut string = String::with_capacity(E_TOKEN_LEN);
        let start = lexer.start();

        string.push_str(&lexer.eat(is_bin_digit));

        if let Ok((_, pot)) = lexer.expect(vec!['.']) {
            string.push(pot);
            string.push_str(&lexer.eat(is_bin_digit));
        }

        string
            .parse::<f64>()
            .map_err(|_| LexerErr::panic(format!("could not parse binary"), start))
            .map(|number| Number(number))
    }
}

fn is_dec_digit(pot: &char) -> bool {
    DEC_DIGITS.contains(*pot)
}

fn is_hex_digit(pot: &char) -> bool {
    HEX_DIGITS.contains(*pot)
}

fn is_bin_digit(pot: &char) -> bool {
    BIN_DIGITS.contains(*pot)
}
