use crate::ast::{Expression, Op, Precedence, Program, Statement};
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
                        // Skipping Assigne Token
                        self.lexer.next();
                    }

                    let next_token = self.lexer.next();
                    if let Some(expression) = self.parse_expression(next_token, Precedence::Lowest)
                    {
                        statements.push(Statement::Let {
                            name: identifier.literal,
                            value: expression,
                        })
                    };
                }
                TokenType::Return => {
                    let next_token = self.lexer.next();
                    if let Some(expression) = self.parse_expression(next_token, Precedence::Lowest)
                    {
                        statements.push(Statement::Return { value: expression });
                    };
                }
                _ => {
                    if let Some(expression) = self.parse_expression(Some(token), Precedence::Lowest)
                    {
                        statements.push(Statement::StatmentExpression { value: expression })
                    }
                }
            }
        }

        dbg!(&statements);
        statements
    }

    pub fn parse_expression(
        &mut self,
        token: Option<Token>,
        precedence: Precedence,
    ) -> Option<Expression> {
        let mut left = match token.clone() {
            Some(t) => match t {
                Token {
                    token_type,
                    literal,
                } => match token_type {
                    TokenType::Number => Expression::Number(literal.parse().unwrap()),
                    TokenType::Minus | TokenType::Bang => {
                        return self.prefix_parser_function(token_type, literal)
                    }
                    _ => return None,
                },
            },

            None => panic!("Line 88"),
        };

        while precedence < self.current_precedence(&token.clone().unwrap()) {
            if let Some(expression) =
                self.infix_parser_function(left.clone(), token.clone().unwrap())
            {
                left = expression
            } else {
                break;
            }
        }

        Some(left)
    }

    pub fn prefix_parser_function(
        &mut self,
        token_type: TokenType,
        literal: String,
    ) -> Option<Expression> {
        let next_token = self.lexer.next();
        let clone_of_token_type = token_type.clone();
        Some(Expression::PrefixExpression {
            Token: Token::new(token_type, literal),
            Op: Op::token(&clone_of_token_type),
            Right: Box::new(self.parse_expression(next_token, Precedence::Lowest)),
        })
    }

    pub fn infix_parser_function(&mut self, left: Expression, token: Token) -> Option<Expression> {
        let next_token = self.lexer.next();
        let precedence = self.peek_precedence();
        dbg!(&token.token_type);
        Some(Expression::InfixExpression {
            Token: token.clone(),
            Left: Box::new(left),
            Op: Op::token(&token.token_type),
            Right: Box::new(self.parse_expression(next_token, precedence)),
        })
    }

    fn peek_precedence(&mut self) -> Precedence {
        if let Some(token) = self.lexer.peek() {
            Precedence::get_precedence(&token.token_type)
        } else {
            Precedence::Lowest
        }
    }

    fn current_precedence(&self, token: &Token) -> Precedence {
        Precedence::get_precedence(&token.token_type)
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
    fn parse_infix_expression() {
        let lexer = lexer::Lexer::new(String::from("5 + 5"));
        let mut parser = Parser::new(lexer);
        let program = parser.parser();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::InfixExpression {
                Token: Token {
                    token_type: TokenType::Addition,
                    literal: "+".to_string(),
                },
                Left: Box::new(Expression::Number(5.0)),
                Op: Op::Add,
                Right: Box::new(Some(Expression::Number(5.0))),
            },
        }]);

        assert_eq!(program, expected_program)
    }

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
                    Op: Op::Subtract,
                    Right: Box::new(Some(Expression::Number(123.0))),
                },
            },
            Statement::StatmentExpression {
                value: Expression::PrefixExpression {
                    Token: Token {
                        token_type: TokenType::Bang,
                        literal: "!".to_string(),
                    },
                    Op: Op::Bang,
                    Right: Box::new(Some(Expression::Number(124.0))),
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
