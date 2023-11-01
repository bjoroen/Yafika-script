use crate::ast::{self, Expression, Node, Statement};

use super::object;

pub fn eval(node: Node) -> object::Object {
    match node {
        Node::Program(p) => eval_program(p),
        Node::Statment(s) => eval_statment(s),
        Node::Expression(e) => eval_expression(e),
        Node::BlockStatment(b) => eval_block_statment(b),
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
        _ => todo!(),
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

    fn test_eval(test_case: &[(&str, object::Object)]) {
        for (input, expected) in test_case {
            let lexer = lexer::Lexer::new(String::from(*input));
            let mut parser = Parser::new(lexer);
            parser.read();
            parser.read();
            let program = parser.parse();
            assert_eq!(eval(ast::Node::Program(program)), *expected);
        }
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

    #[test]
    fn evaluate_boolean_expression() {
        let test_case = [
            ("True ", object::Object::Boolean(true)),
            ("False ", object::Object::Boolean(false)),
        ];

        test_eval(&test_case)
    }
}
