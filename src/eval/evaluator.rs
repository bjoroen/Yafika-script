use crate::ast::{self, Expression, Node, Op, Statement};

#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

use super::object::{self, EvalError, Object};

pub fn eval(node: Node) -> Result<Object, EvalError> {
    match node {
        Node::Program(p) => eval_program(p),
        Node::BlockStatment(b) => eval_program(b.Statement),
        Node::Statment(s) => eval_statment(s),
        Node::Expression(e) => eval_expression(e),
    }
}

fn eval_program(p: Vec<Statement>) -> Result<Object, EvalError> {
    let mut result: object::Object = object::Object::Nil;
    for statment in p {
        let stmt = eval_statment(statment);
        match stmt {
            Ok(s) => match s {
                Object::Return(_) => return Ok(s),
                _ => result = s,
            },
            Err(e) => return Err(e),
        };
    }
    Ok(result)
}

fn eval_statment(s: Statement) -> Result<Object, EvalError> {
    match s {
        Statement::Let { name: _, value: _ } => todo!(),
        Statement::Return { value: v } => {
            let value = eval_expression(v)?;
            return Ok(object::Object::Return(Box::new(value)));
        }
        Statement::StatmentExpression { value } => eval_expression(value),
    }
}

fn eval_expression(e: Expression) -> Result<Object, EvalError> {
    match e {
        Expression::Number(n) => Ok(Object::Integer(n)),
        Expression::Boolean(b) => Ok(Object::Boolean(b)),
        Expression::PrefixExpression {
            Token: _,
            Op,
            Right,
        } => {
            let right = eval_expression(Right.expect("eval prefix"))?;
            eval_prefix(Op, right)
        }
        Expression::InfixExpression {
            Token: _,
            Left,
            Op,
            Right,
        } => {
            let left = eval_expression(*Left)?;
            let right = eval_expression(Right.unwrap())?;

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
) -> Result<Object, EvalError> {
    let con = eval_expression(*condition)?;

    // TODO: Refactor this solution
    if is_truthy(con) {
        eval(ast::Node::BlockStatment(consequence))
    } else {
        match alternative {
            Some(v) => eval(ast::Node::BlockStatment(v)),
            None => Ok(Object::Nil),
        }
    }
}

fn is_truthy(obj: object::Object) -> bool {
    match obj {
        object::Object::Nil => return false,
        object::Object::Boolean(false) => return false,
        _ => true,
    }
}

fn eval_infix_expression(
    left: object::Object,
    op: ast::Op,
    right: object::Object,
) -> Result<Object, EvalError> {
    match (&left, &right) {
        (object::Object::Integer(ln), object::Object::Integer(rn)) => {
            eval_int_infix_expression(ln, op, rn)
        }
        (object::Object::Boolean(lb), object::Object::Boolean(rb)) => {
            eval_bool_infix_expression(lb, op, rb)
        }
        _ => Err(format!(
            "type mismatch: {} {} {}",
            left.type_info(),
            op,
            right.type_info()
        )),
    }
}

fn eval_bool_infix_expression(lb: &bool, op: ast::Op, rb: &bool) -> Result<Object, EvalError> {
    match op {
        Op::Equals => Ok(Object::Boolean(lb == rb)),
        Op::NotEquals => Ok(Object::Boolean(lb != rb)),
        _ => Err(format!("unknown operator: {} {} {}", lb, op, rb)),
    }
}

fn eval_int_infix_expression(ln: &f64, op: ast::Op, rn: &f64) -> Result<Object, EvalError> {
    match op {
        Op::Add => Ok(Object::Integer(ln + rn)),
        Op::Subtract => Ok(Object::Integer(ln - rn)),
        Op::Multiply => Ok(Object::Integer(ln * rn)),
        Op::Divide => Ok(Object::Integer(ln / rn)),
        Op::LessThan => Ok(Object::Boolean(ln > rn)),
        Op::GreaterThan => Ok(Object::Boolean(ln < rn)),
        Op::Equals => Ok(Object::Boolean(ln == rn)),
        Op::NotEquals => Ok(Object::Boolean(ln != rn)),

        _ => Err(format!("unknown operator: {}", op)),
    }
}

fn eval_prefix(op: ast::Op, right: object::Object) -> Result<object::Object, EvalError> {
    match op {
        ast::Op::Bang => eval_bang_prefix(right),
        ast::Op::Subtract => eval_sub_prefix(right),
        _ => Err(format!("unknown operator: {}{}", op, right)),
    }
}

fn eval_sub_prefix(right: Object) -> Result<Object, EvalError> {
    match right {
        object::Object::Integer(i) => Ok(Object::Integer(-i)),
        _ => Err(format!("unknown operator: -{}", right)),
    }
}

fn eval_bang_prefix(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Nil => Ok(Object::Boolean(true)),
        Object::Boolean(b) => Ok(Object::Boolean(!b)),
        _ => Ok(Object::Boolean(false)),
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

            match eval(ast::Node::Program(program)) {
                Ok(v) => p_assert_eq!(v, *expected),
                Err(e) => p_assert_eq!(e, *expected.to_string()),
            }
        }
    }

    #[test]
    fn error_handling() {
        let test_case = [
            (
                "5 + True",
                Object::Error("type mismatch: INT + BOOLEAN".to_string()),
            ),
            (
                "5 + True; 5;",
                Object::Error("type mismatch: INT + BOOLEAN".to_string()),
            ),
            (
                "-True",
                Object::Error("unknown operator: -true".to_string()),
            ),
            (
                "True + False",
                Object::Error("unknown operator: true + false".to_string()),
            ),
            (
                "5 True + False 5",
                Object::Error("unknown operator: true + false".to_string()),
            ),
            (
                "if (10 > 5) { True + False }",
                Object::Error("unknown operator: true + false".to_string()),
            ),
        ];

        test_eval(&test_case)
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
