use std::collections::HashMap;

use crate::{ast::{StatementOrExpression, Expression, Statement, Identifier, VariableDecleration, ObjectLiteral, ArrayLiteral, MemberExpr}, environment::Environment};

#[derive(Debug, Clone)]
pub enum RuntimeVal {
    NumberVal(f64),
    StringVal(String),
    BoolVal(bool),
    ArrayVal(Vec<RuntimeVal>),
    ObjectVal(HashMap<String, RuntimeVal>),
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

fn string_concat(raw_left: RuntimeVal, raw_right: RuntimeVal) -> RuntimeVal {
    // String Concatination
    let left: Option<String> = match raw_left {
        RuntimeVal::StringVal(s) => Some(s),
        RuntimeVal::NumberVal(n) => Some(n.to_string()),
        _ => None,
    };

    let right: Option<String> = match raw_right {
        RuntimeVal::StringVal(s) => Some(s),
        RuntimeVal::NumberVal(n) => Some(n.to_string()),
        _ => None,
    };

    if left.is_some() && right.is_some() {
        return RuntimeVal::StringVal(format!("{}{}", left.unwrap(), right.unwrap()));
    }

    RuntimeVal::NullVal
}

fn string_divide(raw_left: RuntimeVal, raw_right: RuntimeVal) -> RuntimeVal {
    // produces ArrayVal
    let left: Option<String> = match raw_left {
        RuntimeVal::StringVal(s) => Some(s),
        RuntimeVal::NumberVal(n) => Some(n.to_string()),
        _ => None,
    };

    let right: Option<String> = match raw_right {
        RuntimeVal::StringVal(s) => Some(s),
        RuntimeVal::NumberVal(n) => Some(n.to_string()),
        _ => None,
    };

    // split left by right
    if left.is_some() && right.is_some() {
        let mut arr = Vec::new();
        for s in left.unwrap().split(&right.unwrap()) {
            arr.push(RuntimeVal::StringVal(s.to_string()));
        }
        return RuntimeVal::ArrayVal(arr);
    }

    RuntimeVal::NullVal
}

fn eval_binary_expr(raw_left: RuntimeVal, raw_right: RuntimeVal, op: &str) -> RuntimeVal {

    // Math
    let left: Option<f64> = match raw_left {
        RuntimeVal::NumberVal(n) => Some(n),
        _ => None,
    };

    let right: Option<f64> = match raw_right {
        RuntimeVal::NumberVal(n) => Some(n),
        _ => None,
    };

    if left.is_some() && right.is_some() {
        return eval_numeric_binary_expr(left.unwrap(), right.unwrap(), op);
    }

    // String operations
    match op {
        "+" => string_concat(raw_left, raw_right),
        "/" => string_divide(raw_left, raw_right),
        _ => RuntimeVal::NullVal,
    }
}

fn eval_member_expr(expr: MemberExpr, env: &mut Environment) -> RuntimeVal {
    let obj = eval_expr(*expr.object, env);
    let prop = eval_expr(*expr.property, env);

    let prop_str: Option<String> = match prop {
        RuntimeVal::StringVal(s) => Some(s),
        RuntimeVal::NumberVal(n) => Some(n.to_string()),
        _ => None,
    };

    if prop_str.is_some() {
        match obj {
            RuntimeVal::ObjectVal(obj) => {
                let prop = prop_str.unwrap();
                if obj.contains_key(&prop) {
                    return obj.get(&prop).unwrap().clone();
                }
            },
            RuntimeVal::ArrayVal(arr) => {
                let prop = prop_str.unwrap();
                let index: Option<usize> = prop.parse().ok();
                if index.is_some() {
                    let index = index.unwrap();
                    if index < arr.len() {
                        return arr[index].clone();
                    }
                }
            },
            _ => {},
        }
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

fn eval_array_expr(array: ArrayLiteral, env: &mut Environment) -> RuntimeVal {
    let exprs = array.elements;
    let mut vals = Vec::new();
    for expr in exprs {
        let val = eval_expr(expr, env);
        vals.push(val);
    }
    RuntimeVal::ArrayVal(vals)
}

fn eval_object_expr(obj: ObjectLiteral, env: &mut Environment) -> RuntimeVal {
    let mut map = HashMap::new();
    for prop in obj.into_iter() {
        let key = prop.key;

        let val = match prop.value {
            Some(e) => eval_expr(*e, env),
            None => {
                match env.get(&key) {
                    Some(v) => v.clone(),
                    None => panic!("Property {} not defined", key),
                }
            },
        };

        map.insert(key, val);
    }
    RuntimeVal::ObjectVal(map)
}

fn eval_expr(expr: Expression, env: &mut Environment) -> RuntimeVal {
    match expr {
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::ObjectLiteral(obj) => eval_object_expr(obj, env),
        Expression::ArrayLiteral(exprs) => eval_array_expr(exprs, env),
        Expression::NumericLiteral(n) => RuntimeVal::NumberVal(n.value),
        Expression::StringLiteral(s) => RuntimeVal::StringVal(s.value),
        Expression::Member(expr) => eval_member_expr(expr, env),
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
        #[allow(unreachable_patterns)]
        _ => {
            println!("Expression: {:#?} not yet implemented", expr);
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