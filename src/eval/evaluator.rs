use crate::ast::{self, Expression, Node, Op, Statement};

#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

use super::object::{self, Env, EvalError, Object};

pub fn eval(node: Node, ev: &Env) -> Result<Object, EvalError> {
    match node {
        Node::Program(p) => eval_program(p, ev),
        Node::BlockStatment(b) => eval_program(b.Statement, ev),
        Node::Statment(s) => eval_statment(s, ev),
        Node::Expression(e) => eval_expression(e, ev),
    }
}

fn eval_program(p: Vec<Statement>, ev: &Env) -> Result<Object, EvalError> {
    let mut result: object::Object = object::Object::Nil;
    for statment in p {
        let stmt = eval_statment(statment, ev);
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

fn eval_statment(s: Statement, ev: &Env) -> Result<Object, EvalError> {
    match s {
        Statement::Let { name: n, value: v } => {
            let exp = eval_expression(v, ev)?;
            let _ = &ev.try_borrow_mut().unwrap().set(n, exp);

            Ok(Object::Nil)
        }
        Statement::Return { value: v } => {
            let value = eval_expression(v, ev)?;
            return Ok(object::Object::Return(Box::new(value)));
        }
        Statement::StatmentExpression { value } => eval_expression(value, ev),
    }
}

fn eval_expression(e: Expression, ev: &Env) -> Result<Object, EvalError> {
    match e {
        Expression::Number(n) => Ok(Object::Integer(n)),
        Expression::Boolean(b) => Ok(Object::Boolean(b)),
        Expression::Indentifier(i) => {
            let val = ev.borrow().get(&i);
            match val {
                Some(v) => Ok(v),
                None => Err(format!("identifier not found: {}", i)),
            }
        }
        Expression::PrefixExpression {
            Token: _,
            Op,
            Right,
        } => {
            let right = eval_expression(Right.expect("eval prefix"), ev)?;
            eval_prefix(Op, right)
        }
        Expression::InfixExpression {
            Token: _,
            Left,
            Op,
            Right,
        } => {
            let left = eval_expression(*Left, ev)?;
            let right = eval_expression(Right.unwrap(), ev)?;

            eval_infix_expression(left, Op, right)
        }
        Expression::IfExpression {
            Token: _,
            Condition,
            Consequence,
            Alternative,
        } => eval_ifelse_expression(Condition, Consequence, Alternative, ev),
        Expression::FunctionLiteral {
            Token: _,
            Parameters,
            Body,
        } => {
            return Ok(Object::Function {
                Parameters,
                Body,
                env: ev.clone(),
            })
        }

        _ => todo!(),
    }
}

fn eval_ifelse_expression(
    condition: Box<Expression>,
    consequence: ast::BlockStatment,
    alternative: Option<ast::BlockStatment>,
    ev: &Env,
) -> Result<Object, EvalError> {
    let con = eval_expression(*condition, ev)?;

    // TODO: Refactor this solution
    if is_truthy(con) {
        eval(ast::Node::BlockStatment(consequence), ev)
    } else {
        match alternative {
            Some(v) => eval(ast::Node::BlockStatment(v), ev),
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

    use std::{cell::RefCell, rc::Rc};

    use crate::{
        eval::object::{Environment, Object},
        lexer, Parser,
    };

    use super::*;

    fn test_eval(test_case: &[(&str, Object)]) {
        for (input, expected) in test_case {
            let lexer = lexer::Lexer::new(String::from(*input));
            let mut parser = Parser::new(lexer);
            parser.read();
            parser.read();
            let program = parser.parse();

            let ev = Rc::new(RefCell::new(Environment::new()));

            match eval(ast::Node::Program(program), &ev) {
                Ok(v) => p_assert_eq!(v, *expected),
                Err(e) => p_assert_eq!(e, *expected.to_string()),
            }
        }
    }

    // TODO: Refactor tests, so they all check againts strings and not objects
    //
    fn test_eval_string(test_case: &[(&str, &str)]) {
        for (input, expected) in test_case {
            let lexer = lexer::Lexer::new(String::from(*input));
            let mut parser = Parser::new(lexer);
            parser.read();
            parser.read();
            let program = parser.parse();

            let ev = Rc::new(RefCell::new(Environment::new()));

            match eval(ast::Node::Program(program), &ev) {
                Ok(v) => p_assert_eq!(v.to_string(), *expected.to_string()),
                Err(e) => p_assert_eq!(e, *expected.to_string()),
            }
        }
    }

    #[test]
    fn evaluate_function() {
        let test_case = [(" fn(x) {x + 2}", "fn(x) { x+2 }")];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_let() {
        let test_case = [
            ("let a = 5 a", Object::Integer(5.0)),
            ("let a = 5 * 5 a", Object::Integer(25.0)),
            ("let a = 5 let b = a b", Object::Integer(5.0)),
            (
                "let a = 5 let b = a let c = a + b + 5 c",
                Object::Integer(15.0),
            ),
        ];

        test_eval(&test_case)
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
            (
                "foobar",
                Object::Error("identifier not found: foobar".to_string()),
            ),
        ];

        test_eval(&test_case)
    }

    #[test]
    fn evaluate_return() {
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
    fn evaluate_ifelse() {
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
    fn evaluate_prefix() {
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
    fn evaluate_boolean() {
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
    fn evaluate_int() {
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
