use crate::ast::{self, Expression, Node, Statement};

#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

use super::object;

pub fn eval(node: Node) -> object::Object {
    match node {
        Node::Program(p) => eval_program(p),
        Node::Statment(s) => eval_statment(s),
        Node::Expression(e) => eval_expression(e),
        Node::BlockStatment(b) => eval_program(b.Statement),
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
        Statement::Let { name: _, value: _ } => todo!(),
        Statement::Return { value: _ } => todo!(),
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
        Expression::InfixExpression {
            Token: _,
            Left,
            Op,
            Right,
        } => {
            let left = eval_expression(*Left);
            let right = eval_expression(Right.unwrap());

            eval_infix_expression(left, Op, right)
        }
        Expression::IfExpression {
            Token: _,
            Condition,
            Consequence,
            Alternative,
        } => eval_ifelse_expression(Condition, Consequence, Alternative),
        _ => todo!(),
    }
}

fn eval_ifelse_expression(
    condition: Box<Expression>,
    consequence: ast::BlockStatment,
    alternative: Option<ast::BlockStatment>,
) -> object::Object {
    let con = eval_expression(*condition);

    if let Some(exp) = is_truthy(con) {
        match exp {
            true => eval(ast::Node::BlockStatment(consequence)),
            false => match alternative {
                Some(v) => eval(ast::Node::BlockStatment(v)),
                None => object::Object::Nil,
            },
        }
    } else {
        object::Object::Nil
    }
}

fn is_truthy(obj: object::Object) -> Option<bool> {
    match obj {
        object::Object::Boolean(true) => Some(true),
        object::Object::Boolean(false) => Some(false),
        object::Object::Nil => Some(false),
        _ => Some(true),
    }
}

fn eval_infix_expression(
    left: object::Object,
    op: ast::Op,
    right: object::Object,
) -> object::Object {
    match (left, right) {
        (object::Object::Integer(ln), object::Object::Integer(rn)) => {
            eval_int_infix_expression(ln, op, rn)
        }
        (object::Object::Boolean(lb), object::Object::Boolean(rb)) => {
            eval_bool_infix_expression(lb, op, rb)
        }
        _ => object::Object::Nil,
    }
}

fn eval_bool_infix_expression(lb: bool, op: ast::Op, rb: bool) -> object::Object {
    match op {
        ast::Op::Equals => object::Object::Boolean(lb == rb),
        ast::Op::NotEquals => object::Object::Boolean(lb != rb),
        _ => object::Object::Nil,
    }
}

fn eval_int_infix_expression(ln: f64, op: ast::Op, rn: f64) -> object::Object {
    match op {
        ast::Op::Add => object::Object::Integer(ln + rn),
        ast::Op::Subtract => object::Object::Integer(ln - rn),
        ast::Op::Multiply => object::Object::Integer(ln * rn),
        ast::Op::Divide => object::Object::Integer(ln / rn),
        ast::Op::LessThan => object::Object::Boolean(ln > rn),
        ast::Op::GreaterThan => object::Object::Boolean(ln < rn),
        ast::Op::Equals => object::Object::Boolean(ln == rn),
        ast::Op::NotEquals => object::Object::Boolean(ln != rn),

        _ => object::Object::Nil,
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
    match right {
        object::Object::Integer(i) => object::Object::Integer(-i),
        _ => object::Object::Nil,
    }
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
    fn evaluate_ifelse_expression() {
        let test_case = [
            ("if ( True ) { 10 }", object::Object::Integer(10.00)),
            ("if ( False ) { 10 }", object::Object::Nil),
            ("if (1) { 10 }", object::Object::Integer(10.00)),
            ("if (1 < 2) { 10 }", object::Object::Integer(10.00)),
            ("if (1 > 2) { 10 }", object::Object::Nil),
            (
                "if (1 > 2) { 10 } else { 20 }",
                object::Object::Integer(20.00),
            ),
            (
                "if (1 < 2) { 10 } else { 20 }",
                object::Object::Integer(10.00),
            ),
        ];

        test_eval(&test_case)
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
            ("1 < 2", object::Object::Boolean(true)),
            ("1 > 2", object::Object::Boolean(false)),
            ("1 < 1", object::Object::Boolean(false)),
            ("1 > 1", object::Object::Boolean(false)),
            ("1 == 1", object::Object::Boolean(true)),
            ("1 != 1", object::Object::Boolean(false)),
            ("1 == 2", object::Object::Boolean(false)),
            ("1 != 2", object::Object::Boolean(true)),
            ("True == True", object::Object::Boolean(true)),
            ("False == False", object::Object::Boolean(true)),
            ("True == False", object::Object::Boolean(false)),
            ("True != False", object::Object::Boolean(true)),
            ("False != True", object::Object::Boolean(true)),
            ("(1 < 2) == True", object::Object::Boolean(true)),
            ("(1 < 2) == False", object::Object::Boolean(false)),
            ("(1 > 2) == True", object::Object::Boolean(false)),
            ("(1 > 2) == False", object::Object::Boolean(true)),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_int_expression() {
        let test_case = [
            ("5 ", object::Object::Integer(5.00)),
            ("231.00", object::Object::Integer(231.00)),
            ("-5 ", object::Object::Integer(-5.00)),
            ("-231.00", object::Object::Integer(-231.00)),
            ("5 + 5 + 5 + 5 - 10", object::Object::Integer(10.00)),
            ("2 * 2 * 2 * 2 * 2", object::Object::Integer(32.00)),
            ("20 + 2 * -10", object::Object::Integer(0.00)),
            ("50 / 2 * 2 + 10", object::Object::Integer(60.00)),
            ("3 * (3 * 3) + 10", object::Object::Integer(37.00)),
            (
                "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                object::Object::Integer(50.00),
            ),
            // TODO: Fix parser bug - This does not get parsed correcrly.
            // ("-50 + 100 + -50", object::Object::Integer(0.00)),
        ];

        test_eval(&test_case)
    }
}
