/*
The parser takes in the returned tokens from the tokenizer and constructs and AST (Abstract syntax tree)
*/
use super::ast::Node;
use super::token::OperatorPrecedence;
use super::token::Token;
use super::tokenizer::Tokenizer;

#[derive(Debug)]
enum ParseError {
    UnAbleToParse(String),
    InvalidOperator(String),
}

#[derive(Clone, Debug)]
struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(&self, expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let current_token = match lexer.next() {
            Some(token) => token,
            None => {
                return Err(ParseError::UnAbleToParse(
                    "Unable to parse expression".to_string(),
                ));
            }
        };

        Ok(Parser {
            current_token,
            tokenizer: lexer,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        match self.generate_ast(OperatorPrecedence::DefaultZero) {
            Ok(ast) => Ok(ast),
            Err(e) => return Err(e),
        }
    }

    // main method that's called recursively
    fn generate_ast(
        &mut self,
        operator_precedence: OperatorPrecedence,
    ) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while operator_precedence < self.current_token.get_operator_precedence() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr)?;
            left_expr = right_expr;
        }

        Ok(left_expr)
    }

    // Construct AST node for numbers, taking into account
    // negative prefixes while handling parenthesis
    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperatorPrecedence::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(num) => {
                self.get_next_token()?;
                Ok(Node::Number(num))
            }
            Token::LParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperatorPrecedence::DefaultZero)?;
                self.check_parenthesis(Token::RParen)?;
                if self.current_token == Token::LParen {
                    let right = self.generate_ast(OperatorPrecedence::MulDivide)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(ParseError::InvalidOperator("Failed to parse".to_string())),
        }
    }

    // parse token and convert to AST
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        // todo!()
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorPrecedence::AddSubtract)?;
                Ok(Node::Add(Box::new(right_expr), Box::new(left_expr)))
            }

            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorPrecedence::AddSubtract)?;
                Ok(Node::Subtract(Box::new(right_expr), Box::new(left_expr)))
            }

            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorPrecedence::MulDivide)?;
                Ok(Node::Multiply(Box::new(right_expr), Box::new(left_expr)))
            }

            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperatorPrecedence::MulDivide)?;
                Ok(Node::Divide(Box::new(right_expr), Box::new(left_expr)))
            }

            Token::Caret => {
                self.get_next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperatorPrecedence::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }

            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }

    // this retrieves next token and set it to current token on the parser
    fn check_parenthesis(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token != expected {
            return Err(ParseError::InvalidOperator("Invalid parenthesis".into()));
        }
        Ok(())
    }

    //get next token
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }
}
