mod lexer;
mod parser;
use lexer::lex::Lexer;
use parser::parser::Parser;
use std::{env, fs};
use std::time::Instant;

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();

    if paths.is_empty() {
        println!("Please provide a file path!");
    }

    for path in paths {
        let input = match fs::read_to_string(&path) {
            Ok(inp) => inp,
            Err(err) => {
                println!("{}: {}", &path, err);
                continue;
            }
        };

        let lex = Lexer::new(input.clone());
        let mut parser = Parser::new(lex);
        let now = Instant::now();
        let tree = parser.parse(false);
        if let Some(tree) = tree {
            println!("pure packrat: {:?}", Instant::now()-now);
            println!("{}", tree);
        } else {
            println!("failure");
        }

        let lex = Lexer::new(input.clone());
        let mut parser = Parser::new(lex);
        let now = Instant::now();
        let tree = parser.parse(true);
        if let Some(tree) = tree {
            println!("with pratt: {:?}", Instant::now()-now);
            println!("{}", tree);
        } else {
            println!("failure");
        }
        
    }
}
