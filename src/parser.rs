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
                    } else {
                        self.lexer.next();
                    }

                    let next_token = self.lexer.next();
                    let expression = self.parse_expression(next_token);

                    statements.push(Statement::Let {
                        name: identifier.literal,
                        value: expression,
                    })
                }
                TokenType::Return => {
                    let next_token = self.lexer.next();
                    let expression = self.parse_expression(next_token);
                    statements.push(Statement::Return { value: expression });
                }
                _ => {
                    let expression = self.parse_expression(Some(token));
                    statements.push(Statement::StatmentExpression { value: expression })
                }
            }
        }

        dbg!(&statements);
        statements
    }

    pub fn parse_expression(&mut self, token: Option<Token>) -> Expression {
        dbg!(&token);
        match token {
            Some(t) => match t {
                Token {
                    token_type,
                    literal,
                } => match token_type {
                    TokenType::Number => Expression::Number(literal.parse().unwrap()),
                    TokenType::Minus | TokenType::Bang => {
                        self.prefix_parser_function(token_type, literal)
                    }
                    _ => unimplemented!(),
                },
            },

            None => unimplemented!(),
        }
    }

    pub fn prefix_parser_function(&mut self, token_type: TokenType, literal: String) -> Expression {
        let next_token = self.lexer.next();
        Expression::PrefixExpression {
            Token: Token::new(token_type, literal),
            Right: Box::new(self.parse_expression(next_token)),
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
        let lexer = lexer::Lexer::new(String::from("-123 !124"));
        let mut parser = Parser::new(lexer);
        let program = parser.parser();

        let expected_program: ast::Program = Vec::from([
            Statement::StatmentExpression {
                value: Expression::PrefixExpression {
                    Token: Token {
                        token_type: TokenType::Minus,
                        literal: "-".to_string(),
                    },
                    Right: Box::new(Expression::Number(123.0)),
                },
            },
            Statement::StatmentExpression {
                value: Expression::PrefixExpression {
                    Token: Token {
                        token_type: TokenType::Bang,
                        literal: "!".to_string(),
                    },
                    Right: Box::new(Expression::Number(124.0)),
                },
            },
        ]);

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
