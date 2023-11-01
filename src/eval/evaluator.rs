use crate::ast;

use super::object;

pub fn eval(node: ast::Node) -> object::Object {
    match node {
        ast::Node::Program(p) => eval_program(p),
        ast::Node::Statment(s) => eval_statment(s),
        ast::Node::Expression(e) => eval_expression(e),
        ast::Node::BlockStatment(b) => eval_block_statment(b),
    }
}

fn eval_program(p: Vec<ast::Statement>) -> object::Object {
    let mut val: object::Object = object::Object::Nil;
    for statment in p {
        val = eval_statment(statment);
    }
    val
}

fn eval_statment(s: ast::Statement) -> object::Object {
    match s {
        ast::Statement::Let { name, value } => todo!(),
        ast::Statement::Return { value } => todo!(),
        ast::Statement::StatmentExpression { value } => eval_expression(value),
    }
}

fn eval_expression(e: ast::Expression) -> object::Object {
    match e {
        ast::Expression::Number(n) => object::Object::Integer(n),
        ast::Expression::String(_) => todo!(),
        ast::Expression::Indentifier(_) => todo!(),
        ast::Expression::Boolean(_) => todo!(),
        ast::Expression::FunctionLiteral {
            Token,
            Parameters,
            Body,
        } => todo!(),
        ast::Expression::IfExpression {
            Token,
            Condition,
            Consequence,
            Alternative,
        } => todo!(),
        ast::Expression::PrefixExpression { Token, Op, Right } => todo!(),
        ast::Expression::InfixExpression {
            Token,
            Left,
            Op,
            Right,
        } => todo!(),
        ast::Expression::CallExpression {
            Token,
            Function,
            Arguments,
        } => todo!(),
    }
}

fn eval_block_statment(b: ast::BlockStatment) -> object::Object {
    todo!()
}

#[cfg(test)]
mod tests {

    use crate::eval::object::Object;
    use crate::{lexer, Parser};

    use super::*;

    #[test]
    fn evaluate_int_expression() {
        let lexer = lexer::Lexer::new(String::from("5 "));
        let mut parser = Parser::new(lexer);
        parser.read();
        parser.read();
        let program = parser.parse();

        let vec_expression = vec![ast::Statement::StatmentExpression {
            value: ast::Expression::Number(5.00),
        }];

        assert_eq!(program, vec_expression);
        assert_eq!(eval(ast::Node::Program(program)), Object::Integer(5.00))
    }
}
