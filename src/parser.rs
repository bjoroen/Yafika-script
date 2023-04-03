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
                    statements.push(Statement::Return { value: expression });
                }
                _ => {
                    let expression = self.parse_expression();
                    dbg!(&expression);
                    statements.push(Statement::StatmentExpression { value: expression })
                }
            }
        }

        dbg!(&statements);
        statements
    }

    pub fn parse_expression(&mut self) -> Expression {
        match self.lexer.next() {
            Some(Token {
                token_type: TokenType::Number,
                literal,
            }) => Expression::Number(literal.parse().unwrap()),
            Some(Token {
                token_type: TokenType::Minus,
                literal,
            })
            | Some(Token {
                token_type: TokenType::Bang,
                literal,
            }) => self.prefix_parser_function(Token {
                token_type,
                literal,
            }),
            _ => unimplemented!(),
        }
    }

    pub fn prefix_parser_function(&mut self, token: Token) -> Expression {
        Expression::PrefixExpression {
            Token: Token::new(TokenType::Minus, "-".to_string()),
            Right: Box::new(self.parse_expression()),
        }
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
    fn parse_prefix_expression() {
        let lexer = lexer::Lexer::new(String::from("-5"));
        let mut parser = Parser::new(lexer);
        let program = parser.parser();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::PrefixExpression {
                Token: Token {
                    token_type: TokenType::Minus,
                    literal: "-".to_string(),
                },
                Right: Box::new(Expression::Number(5.0)),
            },
        }]);

        assert_eq!(program, expected_program);
    }

    #[test]
    fn parser_let_test() {
        let lexer = lexer::Lexer::new(String::from("let hello = 123"));
        let mut parser = Parser::new(lexer);
        let program = parser.parser();

        let expected_program: ast::Program = Vec::from([Statement::Let {
            name: "hello".to_string(),
            value: (Expression::Number(123.0)),
        }]);

        assert_eq!(program, expected_program);
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
        let program = parser.parser();

        let expected_program: ast::Program = Vec::from([
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

        assert_eq!(program, expected_program)
    }
}
