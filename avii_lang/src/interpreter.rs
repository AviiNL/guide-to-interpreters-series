use crate::ast::{StatementOrExpression, Expression, Statement};

#[derive(Debug)]
pub enum RuntimeVal {
    NumberVal(f64),
    NullVal,
}

fn eval_numeric_binary_expr(left: f64, right: f64, op: &str) -> RuntimeVal {
    match op {
        "+" => RuntimeVal::NumberVal(left + right),
        "-" => RuntimeVal::NumberVal(left - right),
        "*" => RuntimeVal::NumberVal(left * right),
        "/" => {
            if right == 0.0 {
                panic!("Division by zero");
            }
            RuntimeVal::NumberVal(left / right)
        },
        _ => panic!("Unknown operator {}", op),
    }
}

fn eval_binary_expr(left: RuntimeVal, right: RuntimeVal, op: &str) -> RuntimeVal {

    let left: Option<f64> = match left {
        RuntimeVal::NumberVal(n) => Some(n),
        _ => None,
    };

    let right: Option<f64> = match right {
        RuntimeVal::NumberVal(n) => Some(n),
        _ => None,
    };

    if left.is_some() && right.is_some() {
        return eval_numeric_binary_expr(left.unwrap(), right.unwrap(), op);
    }

    RuntimeVal::NullVal
}

fn eval_expr(expr: Expression) -> RuntimeVal {
    match expr {
        Expression::NullLiteral(_) => RuntimeVal::NullVal,
        Expression::NumericLiteral(n) => RuntimeVal::NumberVal(n.value),
        Expression::BinaryExpr(b) => {
            let left = eval_expr(*b.left);
            let right = eval_expr(*b.right);
            let op = b.operator.as_str();
            eval_binary_expr(left, right, op)
        },
        Expression::Identifier(_) => panic!("Identifier not implemented"),
    }
}

fn eval_stmt(stmt: Statement) -> RuntimeVal {
    match stmt {
        Statement::Let(_) => {
            panic!("Not yet implemented");
        },
        Statement::Program(p) => {
            let mut last_val = RuntimeVal::NullVal;
            for stmt in p.body {
                last_val = evaluate(stmt);
            }
            last_val
        },
    }
}

pub fn evaluate(ast_node: StatementOrExpression) -> RuntimeVal {
    match ast_node {
        StatementOrExpression::Expression(expr) => eval_expr(expr),
        StatementOrExpression::Statement(stmt) => eval_stmt(stmt),
    }
}