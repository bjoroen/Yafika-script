use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parser(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token.token_type {
                TokenType::Let => {
                    let identifier = if let Some(identifier) = self.lexer.next() {
                        identifier
                    } else {
                        panic!("Expected identifier")
                    };

                    if !matches!(
                        self.lexer.peek(),
                        Some(Token {
                            token_type: TokenType::Assign,
                            ..
                        })
                    ) {
                        println!("{:?}", self.lexer.peek());
                        panic!("Expected Equal for assigment")
                    }

                    self.lexer.next();

                    let expression = self.parse_expression();

                    statements.push(Statement::Let {
                        name: identifier.literal,
                        value: expression,
                    })
                }
                TokenType::Return => {
                    let expression = self.parse_expression();
                    println!("{:?}", expression);
                    statements.push(Statement::Return { value: expression });
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

    pub fn prefix_parser_function() -> Expression {
        todo!()
    }

    pub fn infix_parser_function(Expr: Expression) -> Expression {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        ast::{self, Expression},
        lexer,
    };

    use super::*;

    #[test]
    fn parser_let_test() {
        let lexer = lexer::Lexer::new(String::from("let hello = 123"));
        let mut parser = Parser::new(lexer);
        let prog = parser.parser();

        let expected_prog: ast::Program = Vec::from([Statement::Let {
            name: "hello".to_string(),
            value: (Expression::Number(123.0)),
        }]);

        assert_eq!(prog, expected_prog);
    }

    #[test]
    fn parse_return_test() {
        let lexer = lexer::Lexer::new(String::from(
            "
                return 123
                return 10
                return 92031203",
        ));
        let mut parser = Parser::new(lexer);
        let prog = parser.parser();

        let expected_prog: ast::Program = Vec::from([
            Statement::Return {
                value: (Expression::Number(123.0)),
            },
            Statement::Return {
                value: (Expression::Number(10.0)),
            },
            Statement::Return {
                value: (Expression::Number(92031203.0)),
            },
        ]);

        assert_eq!(prog, expected_prog)
    }
}
