#[derive(Debug)]
#[derive(PartialEq)]
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
