use crate::ast::{Expression, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::<Statement>::new();

        while let Some(token) = self.lexer.next() {
            match token.token_type {
                TokenType::Let => {
                    let identifier = if let Some(identifer) = self.lexer.next() {
                        identifer
                    } else {
                        panic!("Expected Identifier")
                    };

                    if !matches!(
                        self.lexer.peek(),
                        Some(Token {
                            token_type: TokenType::Assign,
                            ..
                        })
                    ) {
                        panic!("Expected equal for assignment")
                    }

                    self.lexer.next();
                    let expression = self.parse_expression();

                    statements.push(Statement::Let { name: identifier.literal, initial_value: expression })
                }
                _ => unimplemented!(),
            }
        }

        statements
    }

    pub fn parse_expression(&mut self) -> Expression {
        match self.lexer.next() {
            Some(Token {
                token_type: TokenType::Number,
                literal,
            }) => Expression::Number(literal.parse().unwrap()),
            _ => unimplemented!(),
        }
    }
}

pub type Program = Vec<Statement>;

#[cfg(test)]
mod tests {
    use crate::lexer;

    use super::*;

    #[test]
    fn parser_test() {
        let lexer = lexer::Lexer::new(String::from("let hello = 123"));
        let mut parser = Parser::new(lexer);
        let prog = parser.parse();

        let mut expected_prog: Program = Vec::new();

        expected_prog.push(Statement::Let { name: "hello".to_string(), initial_value: (Expression::Number(123.0)) });

        assert_eq!(prog, expected_prog);
    }
}
