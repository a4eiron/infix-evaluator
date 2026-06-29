use std::io::{self, Write};

mod evaluator;
mod lexer;
mod parser;

use lexer::*;
use parser::*;

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    println!("Type 'quit' or 'exit' to exit.");
    loop {
        input.clear();

        print!("calc> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "quit" || input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        match evaluate_infix(input) {
            Some(res) => println!("{res}"),
            None => println!("Error: Invalid expression"),
        }
    }
    Ok(())
}

fn evaluate_infix(expr: &str) -> Option<f64> {
    let tokens = lex(expr)?;
    // println!("{tokens:#?}");

    let mut parser = Parser::new(tokens);
    let ast = parser.parse(0)?;

    if parser.peek().is_some() {
        return None;
    }
    // println!("{ast:#?}");

    let result = evaluator::eval(&ast);

    Some(result)
}
