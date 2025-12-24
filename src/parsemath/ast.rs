pub enum Node {
    Add(Box<Node>, Box<Node>),
    Number(f64),
    Negative(Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
}
