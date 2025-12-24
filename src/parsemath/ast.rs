use std::error;

#[derive(Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Number(f64),
    Negative(Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
}

pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    match expr {
        Node::Number(num) => Ok(num),
        Node::Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Node::Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Node::Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Node::Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Node::Negative(expr) => Ok(-eval(*expr)?),
        Node::Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
    }
}
