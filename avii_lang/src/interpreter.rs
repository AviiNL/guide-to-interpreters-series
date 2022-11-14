use std::{collections::HashMap, fmt::{Debug, Display}};

use crate::{ast::{StatementOrExpression, Expression, Statement, Identifier, VariableDecleration, ObjectLiteral, ArrayLiteral, MemberExpr, CallExpr, FunctionDecleration, FunctionLiteral, Condition}, environment::Environment};

// pub type BuiltInFunc = for<'r, 's> fn(&'r [&'r RuntimeVal]) -> RuntimeVal;
// // trait BuiltInFunc<'a> {}

// // impl<'a, F> BuiltInFunc<'a> for F where F: Fn(&'a [&'a RuntimeVal]) -> RuntimeVal {}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Identifier>,
    pub body: Vec<StatementOrExpression>,
    pub env: Environment,
}

#[derive(Clone)]
pub struct BuiltInFunction {
    pub function: Box<fn(Vec<RuntimeVal>) -> RuntimeVal>,
}

impl Debug for BuiltInFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltInFunction")
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeVal {
    NumberVal(f64),
    StringVal(String),
    BoolVal(bool),
    ArrayVal(Vec<RuntimeVal>),
    ObjectVal(HashMap<String, RuntimeVal>),
    FunctionVal(Function),
    BuiltInFunction(BuiltInFunction),
    NullVal,
}

impl Display for RuntimeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVal::NumberVal(n) => write!(f, "{}", n),
            RuntimeVal::StringVal(s) => write!(f, "{}", s),
            RuntimeVal::BoolVal(b) => write!(f, "{}", b),
            RuntimeVal::ArrayVal(a) => write!(f, "{:?}", a),
            RuntimeVal::ObjectVal(o) => write!(f, "{:?}", o),
            RuntimeVal::BuiltInFunction(b) => write!(f, "{:?}", b),
            RuntimeVal::NullVal => write!(f, "null"),
            _ => write!(f, "unknown"),
        }
    }
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
        "^" => RuntimeVal::NumberVal(left.powf(right)),
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
    let prop = if expr.computed {
        eval_expr(*expr.property, env)
    } else {
        match *expr.property {
            Expression::Identifier(s) => RuntimeVal::StringVal(s.symbol),
            _ => panic!("Invalid property"),
        }
    };

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
            RuntimeVal::StringVal(s) => {
                let prop = prop_str.unwrap();
                let index: usize = match prop.parse() {
                    Ok(i) => i,
                    Err(_) => panic!("Invalid index"),
                };

                if index < s.len() {
                    return RuntimeVal::StringVal(s.chars().nth(index).unwrap().to_string());
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

fn eval_call_expr(call: CallExpr, env: &mut Environment) -> RuntimeVal {
    let callee = eval_expr(*call.caller, env);
    let mut args = Vec::new();
    for arg in call.arguments {
        args.push(eval_expr(arg, env));
    }

    match callee {
        RuntimeVal::FunctionVal(f) => {
            // let mut new_env = Environment::new_with_parent(env.clone());
            let mut new_env = f.env;

            if args.len() != f.params.len() {
                panic!("Invalid number of arguments");
            }

            for (i, arg) in f.params.into_iter().enumerate() {
                new_env.set(&arg.symbol, args[i].clone(), false);
            }

            let mut last_val = RuntimeVal::NullVal;
            for stmt in f.body {
                last_val = evaluate(stmt, &mut new_env);
            }
            last_val
        },
        RuntimeVal::BuiltInFunction(f) => {
            let func = *f.function;
            func(args)
        },
        _ => RuntimeVal::NullVal,
    }
}

fn eval_function_literal(func: FunctionLiteral, env: &mut Environment) -> RuntimeVal {
    RuntimeVal::FunctionVal(Function {
        params: func.params,
        body: func.body,
        env: env.clone(),
        name: "closure".to_string(),
    })
}

fn eval_conditional_expr(cond: Condition, env: &mut Environment) -> RuntimeVal {
    let left = cond.left;
    let right = cond.right;
    let operator = cond.operator;

    let left = eval_expr(*left, env);
    let right = eval_expr(*right, env);

    // Boolean operations
    let left_bool: Option<bool> = match left {
        RuntimeVal::BoolVal(b) => Some(b),
        _ => None,
    };

    let right_bool: Option<bool> = match right {
        RuntimeVal::BoolVal(b) => Some(b),
        _ => None,
    };

    if left_bool.is_some() && right_bool.is_some() {
        return eval_boolean_expr(left_bool.unwrap(), right_bool.unwrap(), operator);
    }

    // Numeric operations
    let left_num: Option<f64> = match left {
        RuntimeVal::NumberVal(n) => Some(n),
        _ => None,
    };

    let right_num: Option<f64> = match right {
        RuntimeVal::NumberVal(n) => Some(n),
        _ => None,
    };

    if left_num.is_some() && right_num.is_some() {
        return eval_numeric_boolean_expr(left_num.unwrap(), right_num.unwrap(), operator);
    }

    // String comparison
    let left_str: Option<String> = match left {
        RuntimeVal::StringVal(s) => Some(s),
        _ => None,
    };

    let right_str: Option<String> = match right {
        RuntimeVal::StringVal(s) => Some(s),
        _ => None,
    };

    if left_str.is_some() && right_str.is_some() {
        return eval_string_boolean_expr(left_str.unwrap(), right_str.unwrap(), operator);
    }

    RuntimeVal::BoolVal(false)
}

fn eval_string_boolean_expr(left: String, right: String, operator: String) -> RuntimeVal {
    match operator.as_str() {
        "==" => RuntimeVal::BoolVal(left == right),
        "!=" => RuntimeVal::BoolVal(left != right),
        _ => panic!("Invalid operator"),
    }
}

fn eval_numeric_boolean_expr(left: f64, right: f64, operator: String) -> RuntimeVal {
    match operator.as_str() {
        "==" => RuntimeVal::BoolVal(left == right),
        "!=" => RuntimeVal::BoolVal(left != right),
        _ => panic!("Invalid operator"),
    }
}

fn eval_boolean_expr(left: bool, right: bool, operator: String) -> RuntimeVal {
    match operator.as_str() {
        "==" => RuntimeVal::BoolVal(left == right),
        "!=" => RuntimeVal::BoolVal(left != right),
        "or" => RuntimeVal::BoolVal(left || right),
        "and" => RuntimeVal::BoolVal(left && right),
        _ => panic!("Invalid operator"),
    }
}

fn eval_expr(expr: Expression, env: &mut Environment) -> RuntimeVal {
    match expr {
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::ObjectLiteral(obj) => eval_object_expr(obj, env),
        Expression::ArrayLiteral(exprs) => eval_array_expr(exprs, env),
        Expression::NumericLiteral(n) => RuntimeVal::NumberVal(n.value),
        Expression::StringLiteral(s) => RuntimeVal::StringVal(s.value),
        Expression::Member(expr) => eval_member_expr(expr, env),
        Expression::Call(expr) => eval_call_expr(expr, env),
        Expression::FunctionLiteral(f) => eval_function_literal(f, env),
        Expression::Condition(cond) => eval_conditional_expr(cond, env),
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

fn eval_function_decleration(raw_func: FunctionDecleration, env: &mut Environment) -> RuntimeVal {
    let name = &raw_func.get_name();
    let func = RuntimeVal::FunctionVal(Function {
        name: name.clone(),
        params: raw_func.params,
        body: raw_func.body,
        env: Environment::new_with_parent(env.clone()),
    });
    env.set(&name, func.clone(), false);
    func
}

fn eval_stmt(stmt: Statement, env: &mut Environment) -> RuntimeVal {
    match stmt {
        Statement::VariableDecleration(var) => {
            return eval_var_decleration(var, env);
        },
        Statement::FunctionDecleration(func) => {
            return eval_function_decleration(func, env);
        },
        Statement::Program(p) => {
            let mut last_val = RuntimeVal::NullVal;
            for stmt in p.body {
                last_val = evaluate(stmt, env);
            }
            last_val
        },
        Statement::If(cond) => {
            let test = match eval_expr(*cond.test, env) {
                RuntimeVal::BoolVal(b) => b,
                _ => panic!("Condition must be a boolean"),
            };

            let mut last_val = RuntimeVal::NullVal;
            if test {
                for stmt in cond.consequent {
                    last_val = evaluate(stmt, env);
                }
            } else {
                if let Some(alt) = cond.alternate {
                    for stmt in alt {
                        last_val = evaluate(stmt, env);
                    }
                }
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