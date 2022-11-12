use std::io::Write;

use avii_lang::parser::Parser;

fn main() {

    let prompt = env!("CARGO_PKG_NAME").to_string() + " " + env!("CARGO_PKG_VERSION");

    println!("{}", prompt);

    // REPL with "> " prompt
    loop {
        let mut input = String::new();
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let ast = Parser::produce_ast(&input);
        println!("{:#?}", ast);
    }



    // let parser = Parser::produce_ast(src);

    // println!("{:#?}", parser);
}
