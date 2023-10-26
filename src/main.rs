mod lexer;
mod packrat_parser;
use lexer::lex::Lexer;
use packrat_parser::parser::PackratParser;
use std::{env, fs};

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

        let lex = Lexer::new(input);
        let mut parse = PackratParser::new(lex);
        let tree = parse.parse();
        if let Some(tree) = tree {
            println!("{}", tree);
        } else {
            println!("None");
        }
    }
}
