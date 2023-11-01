use crate::{
    ast::{self, Expression, Node, Statement},
    token::Token,
};

#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

use super::object;

pub fn eval(node: Node) -> object::Object {
    match node {
        Node::Program(p) => eval_program(p),
        Node::Statment(s) => eval_statment(s),
        Node::Expression(e) => eval_expression(e),
    }
}

fn eval_program(p: Vec<Statement>) -> object::Object {
    let mut val: object::Object = object::Object::Nil;
    for statment in p {
        val = eval_statment(statment);
    }
    val
}

fn eval_statment(s: Statement) -> object::Object {
    match s {
        Statement::Let { name, value } => todo!(),
        Statement::Return { value } => todo!(),
        Statement::StatmentExpression { value } => eval_expression(value),
    }
}

fn eval_expression(e: Expression) -> object::Object {
    match e {
        Expression::Number(n) => object::Object::Integer(n),
        Expression::Boolean(b) => object::Object::Boolean(b),
        Expression::PrefixExpression {
            Token: _,
            Op,
            Right,
        } => {
            let right = eval_expression(Right.expect("eval prefix"));
            eval_prefix(Op, right)
        }
        _ => todo!(),
    }
}

fn eval_prefix(op: ast::Op, right: object::Object) -> object::Object {
    match op {
        ast::Op::Bang => eval_bang_prefix(right),
        ast::Op::Subtract => eval_sub_prefix(right),
        _ => object::Object::Nil,
    }
}

fn eval_sub_prefix(right: object::Object) -> object::Object {
    todo!()
}

fn eval_bang_prefix(right: object::Object) -> object::Object {
    match right {
        object::Object::Nil => object::Object::Boolean(true),
        object::Object::Boolean(b) => object::Object::Boolean(!b),
        _ => object::Object::Boolean(false),
    }
}

#[cfg(test)]
mod tests {

    use crate::{lexer, Parser};

    use super::*;

    fn test_eval(test_case: &[(&str, object::Object)]) {
        for (input, expected) in test_case {
            let lexer = lexer::Lexer::new(String::from(*input));
            let mut parser = Parser::new(lexer);
            parser.read();
            parser.read();
            let program = parser.parse();
            p_assert_eq!(eval(ast::Node::Program(program)), *expected);
        }
    }

    #[test]
    fn evaluate_prefix_expression() {
        let test_case = [
            ("!True ", object::Object::Boolean(false)),
            ("!False ", object::Object::Boolean(true)),
            ("!5 ", object::Object::Boolean(false)),
            ("!!True ", object::Object::Boolean(true)),
            ("!!False ", object::Object::Boolean(false)),
            ("!!5 ", object::Object::Boolean(true)),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_boolean_expression() {
        let test_case = [
            ("True ", object::Object::Boolean(true)),
            ("False ", object::Object::Boolean(false)),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_int_expression() {
        let test_case = [
            ("5 ", object::Object::Integer(5.00)),
            ("231.00", object::Object::Integer(231.00)),
            ("21232131.00", object::Object::Integer(21232131.00)),
        ];

        test_eval(&test_case)
    }
}
