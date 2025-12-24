/*
The parser takes in the returned tokens from the tokenizer and constructs and AST (Abstract syntax tree)
*/
use super::ast::Node;
use super::token::OperatorPrecedence;
use super::token::Token;
use super::tokenizer::Tokenizer;

enum ParserError {
    UnAbleToParse(String),
    InvalidOperator(String),
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(&self, expr: &'a str) -> Result<Self, ParserError> {
        let mut lexer = Tokenizer::new(expr);
        let current_token = match lexer.next() {
            Some(token) => token,
            None => {
                return Err(ParserError::UnAbleToParse(
                    "Unable to parse expression".to_string(),
                ));
            }
        };

        Ok(Parser {
            current_token,
            tokenizer: lexer,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        match self.generate_ast(OperatorPrecedence::DefaultZero) {
            Ok(ast) => Ok(ast),
            Err(e) => return Err(e),
        }
    }

    // main method that's called recursively
    //Todo()
    fn generate_ast(&mut self, oper_precedence: OperatorPrecedence) -> Result<Node, ParserError> {
        todo!()
    }

    // retrieves number tokens
    fn parse_number(&mut self) -> Result<Node, ParserError> {
        todo!()
    }

    // parse token and convert to AST
    fn convert_token_to_node(&mut self, token: Token) -> Result<Node, ParserError> {
        todo!()
    }

    // this retrieves next token and set it to current token on the parser
    fn check_parenthesis(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.current_token != expected {
            return Err(ParserError::InvalidOperator("Invalid parenthesis".into()));
        }
        Ok(())
    }

    //get next token
    fn get_next_token(&mut self) -> Result<(), ParserError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParserError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }
}
