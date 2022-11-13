use crate::{ast::{StatementOrExpression, Expression, Statement, Identifier, VariableDecleration}, environment::Environment};

#[derive(Debug, Clone, Copy)]
pub enum RuntimeVal {
    NumberVal(f64),
    BoolVal(bool),
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
        "%" => RuntimeVal::NumberVal(left % right),
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

fn eval_identifier(symbol: Identifier, env: &mut Environment) -> RuntimeVal {
    let symbol = symbol.symbol;
    let env = env.resolve(&symbol);
    
    match env {
        Some(e) => {
            let val = match e.get(&symbol) {
                Some(v) => v,
                None => RuntimeVal::NullVal,
            };
            val.clone()
        },
        None => panic!("Variable {} not defined", symbol),
    }
}

fn eval_expr(expr: Expression, env: &mut Environment) -> RuntimeVal {
    match expr {
        Expression::NumericLiteral(n) => RuntimeVal::NumberVal(n.value),
        Expression::Binary(b) => {
            let left = eval_expr(*b.left, env);
            let right = eval_expr(*b.right, env);
            let op = b.operator.as_str();
            eval_binary_expr(left, right, op)
        },
        Expression::Assignment(a) => {

            match *a.assignee {
                Expression::Identifier(i) => {
                    let symbol = i.symbol;
                    let value = eval_expr(*a.value, env);
                    env.assign(&symbol, value.clone());
                    value
                },
                _ => panic!("Cannot assign to non-identifier (yet)"),
            }
        },
        Expression::Identifier(ident) => eval_identifier(ident, env),
        _ => {
            println!("Expression: {:?} not yet implemented", expr);
            RuntimeVal::NullVal
        },
    }
}

fn eval_var_decleration(var: VariableDecleration, env: &mut Environment) -> RuntimeVal {
    let value = match var.value {
        Some(v) => eval_expr(v, env),
        None => RuntimeVal::NullVal,
    };

    env.set(&var.identifier.symbol, value.clone(), var.constant);

    value
}

fn eval_stmt(stmt: Statement, env: &mut Environment) -> RuntimeVal {
    match stmt {
        Statement::VariableDecleration(var) => {
            return eval_var_decleration(var, env);
        },
        Statement::Program(p) => {
            let mut last_val = RuntimeVal::NullVal;
            for stmt in p.body {
                last_val = evaluate(stmt, env);
            }
            last_val
        },
    }
}

pub fn evaluate(ast_node: StatementOrExpression, env: &mut Environment) -> RuntimeVal {
    match ast_node {
        StatementOrExpression::Expression(expr) => eval_expr(expr, env),
        StatementOrExpression::Statement(stmt) => eval_stmt(stmt, env),
    }
}