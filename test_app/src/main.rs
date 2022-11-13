use std::io::Write;

use avii_lang::{parser::Parser, interpreter::{self, RuntimeVal}, ast::{StatementOrExpression, Statement}, environment::Environment};

fn main() {
    println!("REPL {}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let ast = Parser::produce_ast(&input);
        
        let mut env = Environment::new();

        env.set("PI", RuntimeVal::NumberVal(3.14159265359));
        env.set("true", RuntimeVal::BoolVal(true));
        env.set("false", RuntimeVal::BoolVal(false));
        env.set("null", RuntimeVal::NullVal);

        let result = interpreter::evaluate(StatementOrExpression::Statement(Statement::Program(ast)), &mut env);

        println!("{:#?}", result);
    }
}
