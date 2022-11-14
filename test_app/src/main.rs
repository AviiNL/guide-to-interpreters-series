// https://www.youtube.com/watch?v=aJ3GpwSBv0s&list=PL_2VhOvlMk4UHGqYCLWc6GO8FaPl8fQTh&index=8&ab_channel=tylerlaceby
// 6:44 -> con't on parser.rs after defining properties and objectliterals in ast.rs



use std::io::Write;

use avii_lang::{
    ast::{Statement, StatementOrExpression},
    environment::Environment,
    interpreter,
    parser::Parser,
};

fn main() {

    // allow passing in a filename as argument for parsing
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let filename = &args[1];

        // read the file
        let source_code = std::fs::read_to_string(filename).expect("Could not read file");

        // parse the file
        let program = Parser::produce_ast(&source_code);

        // run the program
        let mut env = Environment::new().with_default_scope();
        interpreter::evaluate(StatementOrExpression::Statement(Statement::Program(program)), &mut env);

        // print the output
        // print!("{:#?}", program);

        // exit
        std::process::exit(0);
    }

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
