#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LParen,
    RParen,
    EOF, // This is for the end of line for the expression - Myke
}

#[derive(Debug, PartialEq, PartialOrd)]
// This define all the operator precedence levels, from lowest to highest
pub enum OperatorPrecedence {
    DefaultZero,
    AddSubtract,
    MulDivide,
    Power,
    Negative,
}

impl Token {
    pub fn get_operator_precedence(&self) -> OperatorPrecedence {
        match self {
            Token::Add | Token::Subtract => OperatorPrecedence::AddSubtract,
            Token::Multiply | Token::Divide => OperatorPrecedence::MulDivide,
            Token::Caret => OperatorPrecedence::Power,
            _ => OperatorPrecedence::DefaultZero,
        }
    }
}
