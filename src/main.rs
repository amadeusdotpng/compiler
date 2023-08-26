mod lexer;
use lexer::{tokens::Token, lex::Lexer};
use std::{fs, env};
fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();

    if paths.is_empty() {
        println!("Please provide a file path!");
    }

    for path in paths {
        let input = match fs::read_to_string(&path) {
            Ok(inp) => inp,
            Err(err) => { println!("{}: {}", &path, err); continue; }
        };

        let mut lex = Lexer::new(input);
        loop {
            let tok = lex.next_token();
            if let Token::EOF = tok {
                println!("{:?}", tok);
                break;
            } else {
                println!("{:?}", tok);
            }
        }
    }
}
