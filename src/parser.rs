use std::io::Error;

use crate::ast::{BlockStatment, Expression, Op, Precedence, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{self, Token, TokenType};

pub struct Parser {
    lexer: Lexer,
    peek: Token,
    current: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            peek: Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
            current: Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
        }
    }
    pub fn parse(&mut self) -> Program {
        let mut program: Program = Vec::new();

        while let Some(statement) = self.next() {
            // dbg!(&statement);
            program.push(statement);
        }

        program
    }

    pub fn parse_statements(&mut self) -> Option<Statement> {
        let stmt = match self.current.token_type {
            TokenType::Let => {
                // TODO: I want this to fail if Let is not followed by identifier
                let identifier = self.peek.clone();
                self.read();
                if !self.expect_n_peek(TokenType::Assign) {
                    panic!("Expected Assign token")
                }

                self.read();
                if let Some(expression) = self.parse_expression(Precedence::Lowest) {
                    Some(Statement::Let {
                        name: identifier.literal,
                        value: expression,
                    })
                } else {
                    None
                }
            }
            TokenType::Return => {
                self.read();
                if let Some(expression) = self.parse_expression(Precedence::Lowest) {
                    Some(Statement::Return { value: expression })
                } else {
                    None
                }
            }
            _ => {
                if let Some(expression) = self.parse_expression(Precedence::Lowest) {
                    Some(Statement::StatmentExpression { value: expression })
                } else {
                    None
                }
            }
        };

        // dbg!(&stmt);
        self.read();
        stmt
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = match self.current.clone().token_type {
            TokenType::Number => Expression::Number(self.current.literal.parse().unwrap()),
            TokenType::Identifier => Expression::Indentifier(self.current.clone().literal),
            TokenType::Bool => Expression::Boolean(self.current.literal == "True".to_string()),
            TokenType::If => self.parse_if_expressions(),
            TokenType::LeftParen => self.parse_grouped_expresion().unwrap(),
            TokenType::Minus | TokenType::Bang => return self.prefix_parser_function(),
            _ => {
                return None;
            }
        };

        while self.peek.token_type != TokenType::EOF && precedence < self.peek_precedence() {
            if let Some(expression) = self.infix_parser_function(left.clone()) {
                left = expression
            }
            self.read();
        }

        // dbg!(&left);
        Some(left)
    }

    pub fn parse_if_expressions(&mut self) -> Expression {
        if !self.expect_n_peek(TokenType::LeftParen) {
            panic!("Syntax error")
        }

        let condition = self.parse_expression(Precedence::Lowest);

        if !self.expect_n_peek(TokenType::LeftBrace) {
            panic!("Syntax error")
        }

        let consequence = self.parse_block_statment();

        let alternative: Option<BlockStatment> = if self.peek_token_is(TokenType::Else) {
            if self.expect_n_peek(TokenType::LeftBrace) {
                panic!("Syntax error")
            }

            let alternative = self.parse_block_statment();

            Some(alternative)
        } else {
            None
        };

        Expression::IfExpression {
            Token: Token {
                token_type: TokenType::If,
                literal: "if".to_string(),
            },
            Condition: Box::new(condition.expect("Syntax error in condition")),
            Consequence: consequence,
            Alternative: alternative,
        }
    }

    pub fn parse_block_statment(&mut self) -> BlockStatment {
        let mut block = Vec::<Statement>::new();

        self.read();
        let next_token = &self.current;
        let token = next_token.clone();

        while self.current.token_type != TokenType::RightBrace
            && self.current.token_type != TokenType::EOF
        {
            // dbg!(self.lexer.next());
            if let Some(stmt) = self.parse_statements() {
                block.push(stmt)
            }
        }

        BlockStatment {
            Token: token,
            Statement: block,
        }
    }

    pub fn parse_grouped_expresion(&mut self) -> Option<Expression> {
        self.read();
        let expression = self.parse_expression(Precedence::Lowest);

        self.expect_n_peek(TokenType::RightParen);

        expression
    }

    pub fn prefix_parser_function(&mut self) -> Option<Expression> {
        let current = self.current.clone();
        // dbg!(&current);
        self.read();

        Some(Expression::PrefixExpression {
            Token: current.clone(),
            Op: Op::token(&current.token_type),
            Right: Box::new(self.parse_expression(Precedence::Lowest)),
        })
    }

    pub fn infix_parser_function(&mut self, left: Expression) -> Option<Expression> {
        let precedence = self.current_precedence();

        //TODO: FIND OUT: Why do I have to read twice here. makes no sense
        let last_peek_token = self.peek.clone();
        self.read();
        self.read();
        Some(Expression::InfixExpression {
            Left: Box::new(left),
            Op: Op::token(&last_peek_token.token_type),
            Token: last_peek_token,
            Right: Box::new(self.parse_expression(precedence)),
        })
    }

    fn peek_precedence(&mut self) -> Precedence {
        Precedence::get_precedence(&self.peek.token_type)
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::get_precedence(&self.current.token_type)
    }

    fn peek_token_is(&mut self, t: TokenType) -> bool {
        if self.peek.token_type == t {
            true
        } else {
            false
        }
    }

    fn expect_n_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.read();
            true
        } else {
            false
        }
    }

    pub fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.lexer.next() {
            token
        } else {
            // dbg!(&self.current);
            Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            }
        }
    }

    pub fn next(&mut self) -> Option<Statement> {
        if self.current.token_type == TokenType::EOF {
            return None;
        }

        self.parse_statements()
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
        let lexer = lexer::Lexer::new(String::from("5 + 5 EOF a + b * 6"));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let expected_program: ast::Program = Vec::from([
            Statement::StatmentExpression {
                value: Expression::InfixExpression {
                    Token: Token {
                        token_type: TokenType::Addition,
                        literal: "+".to_string(),
                    },
                    Left: Box::new(Expression::Number(5.0)),
                    Op: Op::Add,
                    Right: Box::new(Some(Expression::Number(5.0))),
                },
            },
            Statement::StatmentExpression {
                value: Expression::InfixExpression {
                    Token: Token {
                        token_type: TokenType::Addition,
                        literal: "+".to_string(),
                    },
                    Left: Box::new(Expression::Indentifier("a".to_string())),
                    Op: Op::Add,
                    Right: Box::new(Some(Expression::InfixExpression {
                        Token: Token {
                            token_type: TokenType::Star,
                            literal: "*".to_string(),
                        },
                        Left: Box::new(Expression::Indentifier("b".to_string())),
                        Op: Op::Multiply,
                        Right: Box::new(Some(Expression::Number(6.0))),
                    })),
                },
            },
        ]);

        assert_eq!(program, expected_program)
    }

    #[test]
    fn parse_prefix_expression() {
        let lexer = lexer::Lexer::new(String::from("-123 !124"));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

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
        parser.read();
        parser.read();
        let program = parser.parse();

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
        parser.read();
        parser.read();
        let program = parser.parse();

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
