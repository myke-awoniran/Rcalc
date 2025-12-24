#[derive(Debug, PartialEq)]
pub enum Token {
    Num(f64), // for each number token
    Add,      // for the add operator token
    Subtract,
    Multiply,
    Divide,
    Caret,
    LParen,
    RParen,
    EOF, // This is for the end of line for the expression - Myke
}

pub enum OperatorPrecedence {
    DefaultZero,
    AddSubtract,
    MulDivide,
    Power,
    Negative,
}
