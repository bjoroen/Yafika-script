use crate::ast::{BlockStatment, Expression, Op, Precedence, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

pub struct Parser {
    lexer: Lexer,
    peek: Token,
    current: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            // Starting the parser with the peek token as EOF
            peek: Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },

            // Starting the parser with the current token as EOF
            current: Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
        }
    }

    pub fn parse(&mut self) -> Program {
        dbg!(&self.lexer.tokens);
        let mut program: Program = Vec::new();

        while let Some(statement) = self.next() {
            program.push(statement);
        }

        program
    }

    pub fn parse_statements(&mut self) -> Option<Statement> {
        let stmt = match self.current.token_type {
            TokenType::Let => {
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
            TokenType::Minus | TokenType::Bang => return self.parse_prefix_expression(),
            TokenType::Fn => self.parse_function(),
            _ => {
                return None;
            }
        };

        // dbg!(&left);

        while self.peek.token_type != TokenType::EOF
            && self.peek.token_type != TokenType::SemiColon
            && precedence < self.peek_precedence()
        {
            if let Some(expression) = self.parse_infix_expression(left.clone()) {
                left = expression
            }
        }

        Some(left)
    }

    fn parse_function(&mut self) -> Expression {
        let token = self.current.clone();
        if !self.expect_n_peek(TokenType::LeftParen) {
            panic!("Syntax error, got: {:#?}", self.peek)
        }
        self.read();

        let params = self.pase_fn_parameters();

        let body = self.parse_block_statment();

        Expression::FunctionLiteral {
            Token: token,
            Parameters: params,
            Body: body,
        }
    }

    fn pase_fn_parameters(&mut self) -> Option<Vec<Expression>> {
        let mut identifiers = Vec::<Expression>::new();

        if self.current.token_type == TokenType::RightParen {
            self.read();
            return None;
        }

        identifiers.push(Expression::Indentifier(self.current.literal.clone()));

        while self.peek_token_is(TokenType::Comma) {
            self.read();
            self.read();
            if let Some(ident_exp) = self.parse_expression(Precedence::Lowest) {
                if !matches!(ident_exp, Expression::Indentifier(..)) {
                    panic!("Didnt get identifier, got strange shit")
                }

                identifiers.push(ident_exp)
            }
        }

        if !self.expect_n_peek(TokenType::RightParen) {
            panic!("Syntax error, {:#?}", &self.peek)
        }
        self.read();

        Some(identifiers)
    }

    pub fn parse_if_expressions(&mut self) -> Expression {
        if !self.expect_n_peek(TokenType::LeftParen) {
            panic!("Syntax error")
        }
        self.read();

        let condition = self.parse_expression(Precedence::Lowest);

        self.read();
        if !self.expect_n_peek(TokenType::LeftBrace) {
            panic!("Syntax error, {:#?}", &self.current)
        }

        let consequence = self.parse_block_statment();

        let alternative: Option<BlockStatment> = if self.peek_token_is(TokenType::Else) {
            self.read();
            if !self.expect_n_peek(TokenType::LeftBrace) {
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

        while self.current.token_type != TokenType::RightBrace
            && self.current.token_type != TokenType::EOF
        {
            if let Some(stmt) = self.parse_statements() {
                block.push(stmt)
            }
        }

        BlockStatment { Statement: block }
    }

    pub fn parse_grouped_expresion(&mut self) -> Option<Expression> {
        self.read();
        let expression = self.parse_expression(Precedence::Lowest);

        self.expect_n_peek(TokenType::RightParen);

        expression
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let current = self.current.clone();
        self.read();

        Some(Expression::PrefixExpression {
            Token: current.clone(),
            Op: Op::token(&current.token_type),
            Right: Box::new(self.parse_expression(Precedence::Lowest)),
        })
    }

    pub fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        match self.peek.token_type {
            TokenType::LeftParen => {
                self.read();
                if self.peek.token_type == TokenType::RightParen {
                    Some(Expression::CallExpression {
                        Token: self.current.clone(),
                        Function: Box::new(left),
                        Arguments: vec![],
                    })
                } else {
                    let mut args = Vec::<Option<Expression>>::new();

                    let expression_token = self.current.clone();
                    self.read();

                    args.push(self.parse_expression(Precedence::Lowest));

                    while self.peek_token_is(TokenType::Comma) {
                        self.read();
                        self.read();
                        if let Some(expr) = self.parse_expression(Precedence::Lowest) {
                            args.push(Some(expr))
                        }
                    }

                    Some(Expression::CallExpression {
                        Token: expression_token,
                        Function: Box::new(left),
                        Arguments: args,
                    })
                }
            }
            _ => {
                let precedence = self.peek_precedence();
                let last_peek_token = self.peek.clone();

                self.read();
                self.read();
                let right = self.parse_expression(precedence);

                Some(Expression::InfixExpression {
                    Left: Box::new(left),
                    Op: Op::token(&last_peek_token.token_type),
                    Token: last_peek_token,
                    Right: Box::new(right),
                })
            }
        }
    }

    /// Looks at the precedence of the next token type
    fn peek_precedence(&mut self) -> Precedence {
        Precedence::get_precedence(&self.peek.token_type)
    }

    /// Looks at the precedence of the current token type
    fn current_precedence(&self) -> Precedence {
        Precedence::get_precedence(&self.current.token_type)
    }

    /// Given a token token type, returns a bool depending if the next token type is of that type
    fn peek_token_is(&mut self, t: TokenType) -> bool {
        if self.peek.token_type == t {
            true
        } else {
            false
        }
    }

    /// Given a TokenType returns a bool if its the correct token type, this fuction eats the next
    /// token if it is the expected type
    fn expect_n_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.read();
            true
        } else {
            false
        }
    }

    /// Reads and eats the next token
    pub fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.lexer.next() {
            token
        } else {
            Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            }
        }
    }

    /// Returns the next token
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
    fn parse_call_expressionss() {
        let lexer = lexer::Lexer::new(String::from("add(1, 2 * 3, 4 + 5)"));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::CallExpression {
                Token: Token {
                    token_type: TokenType::LeftParen,
                    literal: "(".to_string(),
                },
                Function: Box::new(Expression::Indentifier("add".to_string())),
                Arguments: vec![
                    Some(Expression::Number(1.00)),
                    Some(Expression::InfixExpression {
                        Token: Token {
                            token_type: TokenType::Star,
                            literal: "*".to_string(),
                        },
                        Left: Box::new(Expression::Number(2.00)),
                        Op: Op::Multiply,
                        Right: Box::new(Some(Expression::Number(3.00))),
                    }),
                    Some(Expression::InfixExpression {
                        Token: Token {
                            token_type: TokenType::Addition,
                            literal: "+".to_string(),
                        },
                        Left: Box::new(Expression::Number(4.00)),
                        Op: Op::Add,
                        Right: Box::new(Some(Expression::Number(5.00))),
                    }),
                ],
            },
        }]);

        p_assert_eq!(program, expected_program)
    }

    #[test]
    fn parse_call_no_args_expressionss() {
        let lexer = lexer::Lexer::new(String::from("add() "));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::CallExpression {
                Token: Token {
                    token_type: TokenType::LeftParen,
                    literal: "(".to_string(),
                },
                Function: Box::new(Expression::Indentifier("add".to_string())),
                Arguments: vec![],
            },
        }]);

        p_assert_eq!(program, expected_program)
    }

    #[test]
    fn parse_fn_literals_no_args() {
        let lexer = lexer::Lexer::new(String::from("fn(){let x = a + b; return x}"));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::FunctionLiteral {
                Token: Token {
                    token_type: TokenType::Fn,
                    literal: "fn".to_string(),
                },
                Parameters: None,
                Body: BlockStatment {
                    Statement: vec![
                        Statement::Let {
                            name: "x".to_string(),
                            value: Expression::InfixExpression {
                                Token: Token {
                                    token_type: TokenType::Addition,
                                    literal: "+".to_string(),
                                },
                                Left: Box::new(Expression::Indentifier("a".to_string())),
                                Op: Op::Add,
                                Right: Box::new(Some(Expression::Indentifier("b".to_string()))),
                            },
                        },
                        Statement::Return {
                            value: Expression::Indentifier("x".to_string()),
                        },
                    ],
                },
            },
        }]);

        assert_eq!(program, expected_program)
    }

    #[test]
    fn parse_fn_literals_with_args() {
        let lexer = lexer::Lexer::new(String::from("fn(a,b){let x = a + b; return x}"));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::FunctionLiteral {
                Token: Token {
                    token_type: TokenType::Fn,
                    literal: "fn".to_string(),
                },
                Parameters: Some(vec![
                    Expression::Indentifier("a".to_string()),
                    Expression::Indentifier("b".to_string()),
                ]),
                Body: BlockStatment {
                    Statement: vec![
                        Statement::Let {
                            name: "x".to_string(),
                            value: Expression::InfixExpression {
                                Token: Token {
                                    token_type: TokenType::Addition,
                                    literal: "+".to_string(),
                                },
                                Left: Box::new(Expression::Indentifier("a".to_string())),
                                Op: Op::Add,
                                Right: Box::new(Some(Expression::Indentifier("b".to_string()))),
                            },
                        },
                        Statement::Return {
                            value: Expression::Indentifier("x".to_string()),
                        },
                    ],
                },
            },
        }]);

        assert_eq!(program, expected_program)
    }

    #[test]
    fn parse_if_and_ifelse_expression() {
        let lexer = lexer::Lexer::new(String::from("if(2 > 5) { let x = 2} else {let x = 4}"));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let expected_program: ast::Program = Vec::from([Statement::StatmentExpression {
            value: Expression::IfExpression {
                Token: Token {
                    token_type: TokenType::If,
                    literal: "if".to_string(),
                },
                Condition: Box::new(Expression::InfixExpression {
                    Token: Token {
                        token_type: TokenType::Less,
                        literal: ">".to_string(),
                    },
                    Left: Box::new(Expression::Number(2.0)),
                    Op: Op::LessThan,
                    Right: Box::new(Some(Expression::Number(5.0))),
                }),
                Consequence: BlockStatment {
                    Statement: vec![Statement::Let {
                        name: "x".to_string(),
                        value: Expression::Number(2.0),
                    }],
                },
                Alternative: Some(BlockStatment {
                    Statement: vec![Statement::Let {
                        name: "x".to_string(),
                        value: Expression::Number(4.0),
                    }],
                }),
            },
        }]);

        assert_eq!(program, expected_program)
    }

    #[test]
    fn parse_infix_expression() {
        let lexer = lexer::Lexer::new(String::from(
            "5 + 5
             a + b * 6",
        ));
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

        p_assert_eq!(program, expected_program)
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
    fn parse_let() {
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

    // #[test]
    // fn parse_let_string() {
    //     let lexer = lexer::Lexer::new(String::from("let hello = \"Hello World \""));
    //     let mut parser = Parser::new(lexer);
    //     parser.read();
    //     parser.read();
    //     let program = parser.parse();
    //
    //     let expected_program: ast::Program = Vec::from([Statement::Let {
    //         name: "hello".to_string(),
    //         value: (Expression::Number(123.0)),
    //     }]);
    //
    //     assert_eq!(program, expected_program);
    // }

    #[test]
    fn parse_return() {
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
