use crate::ast::{self, Expression, Node, Statement};

#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

use super::object;

pub fn eval(node: Node) -> object::Object {
    match node {
        Node::Program(p) => eval_program(p),
        Node::BlockStatment(b) => eval_program(b.Statement),
        Node::Statment(s) => eval_statment(s),
        Node::Expression(e) => eval_expression(e),
    }
}

fn eval_program(p: Vec<Statement>) -> object::Object {
    let mut result: object::Object = object::Object::Nil;
    for statment in p {
        let stmt = eval_statment(statment);
        match stmt {
            object::Object::Return(_) => return stmt,
            _ => result = stmt,
        };
    }
    return result;
}

fn eval_statment(s: Statement) -> object::Object {
    match s {
        Statement::Let { name: _, value: _ } => todo!(),
        Statement::Return { value: v } => {
            let value = eval_expression(v);
            object::Object::Return(Box::new(value))
        }
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

    // TODO: Refactor this solution
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

    use crate::{eval::object::Object, lexer, Parser};

    use super::*;

    fn test_eval(test_case: &[(&str, Object)]) {
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
    fn evaluate_return_statments() {
        let test_case = [
            (
                "return 10;",
                Object::Return(Box::new(Object::Integer(10.00))),
            ),
            (
                "return 10; 9:",
                Object::Return(Box::new(Object::Integer(10.00))),
            ),
            (
                "return 2 * 5; 9:",
                Object::Return(Box::new(Object::Integer(10.00))),
            ),
            (
                "9 return 2 * 5; 9:",
                Object::Return(Box::new(Object::Integer(10.00))),
            ),
            (
                "if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }",
                Object::Return(Box::new(Object::Integer(10.00))),
            ),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_ifelse_expression() {
        let test_case = [
            ("if ( True ) { 10 }", Object::Integer(10.00)),
            ("if ( False ) { 10 }", Object::Nil),
            ("if (1) { 10 }", Object::Integer(10.00)),
            ("if (1 < 2) { 10 }", Object::Integer(10.00)),
            ("if (1 > 2) { 10 }", Object::Nil),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20.00)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10.00)),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_prefix_expression() {
        let test_case = [
            ("!True ", Object::Boolean(false)),
            ("!False ", Object::Boolean(true)),
            ("!5 ", Object::Boolean(false)),
            ("!!True ", Object::Boolean(true)),
            ("!!False ", Object::Boolean(false)),
            ("!!5 ", Object::Boolean(true)),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_boolean_expression() {
        let test_case = [
            ("True ", Object::Boolean(true)),
            ("False ", Object::Boolean(false)),
            ("1 < 2", Object::Boolean(true)),
            ("1 > 2", Object::Boolean(false)),
            ("1 < 1", Object::Boolean(false)),
            ("1 > 1", Object::Boolean(false)),
            ("1 == 1", Object::Boolean(true)),
            ("1 != 1", Object::Boolean(false)),
            ("1 == 2", Object::Boolean(false)),
            ("1 != 2", Object::Boolean(true)),
            ("True == True", Object::Boolean(true)),
            ("False == False", Object::Boolean(true)),
            ("True == False", Object::Boolean(false)),
            ("True != False", Object::Boolean(true)),
            ("False != True", Object::Boolean(true)),
            ("(1 < 2) == True", Object::Boolean(true)),
            ("(1 < 2) == False", Object::Boolean(false)),
            ("(1 > 2) == True", Object::Boolean(false)),
            ("(1 > 2) == False", Object::Boolean(true)),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_int_expression() {
        let test_case = [
            ("5 ", Object::Integer(5.00)),
            ("231.00", Object::Integer(231.00)),
            ("-5 ", Object::Integer(-5.00)),
            ("-231.00", Object::Integer(-231.00)),
            ("5 + 5 + 5 + 5 - 10", Object::Integer(10.00)),
            ("2 * 2 * 2 * 2 * 2", Object::Integer(32.00)),
            ("20 + 2 * -10", Object::Integer(0.00)),
            ("50 / 2 * 2 + 10", Object::Integer(60.00)),
            ("3 * (3 * 3) + 10", Object::Integer(37.00)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50.00)),
            // TODO: Fix parser bug - This does not get parsed correcrly.
            // ("-50 + 100 + -50", object::Object::Integer(0.00)),
        ];

        test_eval(&test_case)
    }
}
