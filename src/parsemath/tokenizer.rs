use super::token::Token;
use ::std::iter::Peekable;
use ::std::str::Chars;

/*Note the Tokenizer has two public methods: new and next
 This new mthod creates new instance of the Tokenizer struct
 The next method reads the characters in the expression and returns the next token
*/

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self {
            expr: expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..'9') => {
                let mut digit = next_char?.to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        digit.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }
                Some(Token::Num(digit.parse().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            None => Some(Token::EOF),
            _ => None,
        }
    }
}
