use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{self, Expression, Node, Op, Statement},
    eval::environment::Environment,
};

#[cfg(test)]
use pretty_assertions::assert_eq as p_assert_eq;

use super::{
    environment::Env,
    object::{EvalError, Object},
};

pub fn eval(node: Node, ev: &Env) -> Result<Object, EvalError> {
    match node {
        Node::Program(p) => eval_program(p, ev),
        Node::BlockStatment(b) => eval_program(b.Statement, ev),
        Node::Statment(s) => eval_statment(s, ev),
        Node::Expression(e) => eval_expression(e, ev),
    }
}

fn eval_program(p: Vec<Statement>, ev: &Env) -> Result<Object, EvalError> {
    let mut result: Object = Object::Nil;
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
            //TODO: Match to check if identifier ??

            let exp = eval_expression(v, ev)?;
            let _ = &ev.try_borrow_mut().unwrap().set(n, exp);

            Ok(Object::Nil)
        }
        Statement::Return { value: v } => {
            let value = eval_expression(v, ev)?;
            Ok(Object::Return(Box::new(value)))
        }
        Statement::StatmentExpression { value } => eval_expression(value, ev),
    }
}

fn eval_expression(e: Expression, ev: &Env) -> Result<Object, EvalError> {
    match e {
        Expression::Number(n) => Ok(Object::Integer(n)),
        Expression::String(s) => Ok(Object::String(s)),
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
        Expression::CallExpression {
            Token: _,
            Function,
            Arguments,
        } => {
            let func = eval_expression(*Function, ev)?;

            let mut args = vec![];
            if let Some(expr) = Arguments {
                for exp in expr {
                    args.push(eval_expression(exp, ev)?)
                }
            }

            let mut enclosed_env = Environment::new_enclosed_environment(ev);

            match func {
                Object::Function {
                    Parameters,
                    Body,
                    env: _,
                } => {
                    if let Some(param) = Parameters {
                        param
                            .iter()
                            .enumerate()
                            .for_each(|(indx, param)| match param {
                                Expression::Indentifier(n) => {
                                    enclosed_env.set(n.clone(), args[indx].clone())
                                }
                                _ => todo!("Not sure what to do here yet?"),
                            });
                    }

                    let evaluated = eval(
                        Node::BlockStatment(Body),
                        &Rc::new(RefCell::new(enclosed_env)),
                    )?;

                    return unwrap_return_value(evaluated);
                }
                _ => Err(format!("Expected function")),
            }
        }
    }
}

fn unwrap_return_value(obj: Object) -> Result<Object, EvalError> {
    if let Object::Return(v) = obj {
        Ok(*v)
    } else {
        Ok(obj)
    }
}

fn eval_ifelse_expression(
    condition: Box<Expression>,
    consequence: ast::BlockStatment,
    alternative: Option<ast::BlockStatment>,
    ev: &Env,
) -> Result<Object, EvalError> {
    let condition = eval_expression(*condition, ev)?;

    // TODO: Refactor this solution
    if is_truthy(condition) {
        eval(ast::Node::BlockStatment(consequence), ev)
    } else {
        match alternative {
            Some(v) => eval(ast::Node::BlockStatment(v), ev),
            None => Ok(Object::Nil),
        }
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Nil => return false,
        Object::Boolean(false) => return false,
        _ => true,
    }
}

fn eval_infix_expression(left: Object, op: ast::Op, right: Object) -> Result<Object, EvalError> {
    match (&left, &right) {
        (Object::Integer(ln), Object::Integer(rn)) => eval_int_infix_expression(ln, op, rn),
        (Object::Boolean(lb), Object::Boolean(rb)) => eval_bool_infix_expression(lb, op, rb),
        (Object::String(ls), Object::String(rs)) => eval_string_infix_expression(ls, op, rs),
        _ => Err(format!(
            "type mismatch: {} {} {}",
            left.type_info(),
            op,
            right.type_info()
        )),
    }
}

fn eval_string_infix_expression(ls: &str, op: Op, rs: &str) -> Result<Object, EvalError> {
    match op {
        Op::Add => Ok(Object::String(String::from(ls.to_owned() + rs))),
        _ => Err(format!(
            "unknown operator: {} {} {}",
            String::from("STRING"),
            op,
            String::from("STRING")
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
        Op::LessThan => Ok(Object::Boolean(ln < rn)),
        Op::GreaterThan => Ok(Object::Boolean(ln > rn)),
        Op::Equals => Ok(Object::Boolean(ln == rn)),
        Op::NotEquals => Ok(Object::Boolean(ln != rn)),

        _ => Err(format!("unknown operator: {}", op)),
    }
}

fn eval_prefix(op: ast::Op, right: Object) -> Result<Object, EvalError> {
    match op {
        ast::Op::Bang => eval_bang_prefix(right),
        ast::Op::Subtract => eval_sub_prefix(right),
        _ => Err(format!("unknown operator: {}{}", op, right)),
    }
}

fn eval_sub_prefix(right: Object) -> Result<Object, EvalError> {
    match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
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

    use crate::{lexer, Parser};

    use super::*;

    fn test_eval_string(test_case: &[(&str, &str)]) {
        for (input, expected) in test_case {
            let lexer = lexer::Lexer::new(String::from(*input));
            let mut parser = Parser::new(lexer);
            parser.read();
            parser.read();
            let program = parser.parse();

            let ev: Env = Rc::new(RefCell::new(Default::default()));

            match eval(ast::Node::Program(program), &ev) {
                Ok(v) => p_assert_eq!(v.to_string(), *expected.to_string()),
                Err(e) => p_assert_eq!(e, *expected.to_string()),
            }
        }
    }

    #[test]
    fn evaluate_builtin_functions() {
        let test_case = [
            ("len(\"\")", "0"),
            ("len(\"four\")", "4"),
            ("len(\"hello world\")", "11"),
            ("len(1)", "argument to 'len' not supported, got INT"),
            (
                "len(\"one\", \"two\")",
                "wrong number of arguments. got=2, want=1",
            ),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_string_concatenation() {
        let test_case = [("\"hello\" + \" \" + \"world\"", "hello world")];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_function_application() {
        let test_case = [
            ("let identity = fn(x) {x} identity(1)", "1"),
            ("let identity = fn(x) {return x} identity(2)", "2"),
            ("let double = fn(x) { x * 2 } double(5)", "10"),
            ("let add = fn(x, y) { x + y } add(5, 10)", "15"),
            ("let add = fn(x, y) { x + y } add(5 + 5, add(5, 5))", "20"),
            ("fn(x) {x}(5)", "5"),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_function() {
        let test_case = [(" fn(x) {x + 2}", "fn(x) { x+2 }")];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_let() {
        let test_case = [
            ("let a = 5 a", "5"),
            ("let a = 5 * 5 a", "25"),
            ("let a = 5 let b = a b", "5"),
            ("let a = 5 let b = a let c = a + b + 5 c", "15"),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn error_handling() {
        let test_case = [
            ("5 + True", "type mismatch: INT + BOOLEAN"),
            ("5 + True; 5;", "type mismatch: INT + BOOLEAN"),
            ("-True", "unknown operator: -true"),
            ("True + False", "unknown operator: true + false"),
            ("5 True + False 5", "unknown operator: true + false"),
            (
                "\"hello\" - \" \" - \"world\"",
                "unknown operator: STRING - STRING",
            ),
            (
                "if (10 > 5) { True + False }",
                "unknown operator: true + false",
            ),
            ("foobar", "identifier not found: foobar"),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_return() {
        let test_case = [
            ("return 1;", "1"),
            ("return 2; 9:", "2"),
            ("return 1 * 3; 9:", "3"),
            ("9 return 1 * 4; 9:", "4"),
            (
                "if (10 > 1) {
                    if (10 > 1) {
                        return 10
                    }
                    return 1
                }",
                "10",
            ),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_ifelse() {
        let test_case = [
            ("if ( True ) { 1 }", "1"),
            ("if ( False ) { 2 }", "null"),
            ("if (1) { 3 }", "3"),
            ("if (1 < 2) { 4 }", "4"),
            ("if (1 > 2) { 5 }", "null"),
            ("if (1 > 2) { 6 } else { 7 }", "7"),
            ("if (1 < 2) { 8 } else { 9 }", "8"),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_prefix() {
        let test_case = [
            ("!True ", "false"),
            ("!False ", "true"),
            ("!5 ", "false"),
            ("!!True ", "true"),
            ("!!False ", "false"),
            ("!!5 ", "true"),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_boolean() {
        let test_case = [
            ("True ", "true"),
            ("False ", "false"),
            ("1 < 2", "true"),
            ("1 > 2", "false"),
            ("1 < 1", "false"),
            ("1 > 1", "false"),
            ("1 == 1", "true"),
            ("1 != 1", "false"),
            ("1 == 2", "false"),
            ("1 != 2", "true"),
            ("True == True", "true"),
            ("False == False", "true"),
            ("True == False", "false"),
            ("True != False", "true"),
            ("False != True", "true"),
            ("(1 < 2) == True", "true"),
            ("(1 < 2) == False", "false"),
            ("(1 > 2) == True", "false"),
            ("(1 > 2) == False", "true"),
        ];

        test_eval_string(&test_case)
    }

    #[test]
    fn evaluate_int() {
        let test_case = [
            ("5 ", "5"),
            ("231.00", "231"),
            ("-5 ", "-5"),
            ("-231.00", "-231"),
            ("5 + 5 + 5 + 5 - 10", "10"),
            ("2 * 2 * 2 * 2 * 2", "32"),
            ("20 + 2 * -10", "0"),
            ("50 / 2 * 2 + 10", "60"),
            ("3 * (3 * 3) + 10", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", "50"),
            // TODO: Fix parser bug - This does not get parsed correcrly.
            // ("-50 + 100 + -50", "0"),
        ];

        test_eval_string(&test_case)
    }
}
