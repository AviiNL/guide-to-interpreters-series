use std::io::Write;

use avii_lang::{
    ast::{Statement, StatementOrExpression},
    environment::Environment,
    interpreter,
    parser::Parser,
};

fn main() {
    println!(
        "REPL {}-{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    let mut env = Environment::new().with_default_scope();

    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let ast = Parser::produce_ast(&input);

        let result = interpreter::evaluate(
            StatementOrExpression::Statement(Statement::Program(ast)),
            &mut env,
        );

        println!("{:#?}", result);
    }
}
