mod lexer;
mod parser;
use lexer::lex::Lexer;
use parser::parser::Parser;
use std::{time::Instant, env, fs};

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();

    if paths.is_empty() {
        println!("Please provide a file path!");
    }

    for path in paths {
        let input = match fs::read_to_string(&path) {
            Ok(inp) => String::from(inp.trim_end()),
            Err(err) => {
                println!("{}: {}", &path, err);
                continue;
            }
        };

        let lex = Lexer::new(input.clone());
        let mut parser = Parser::new(lex);
        let now = Instant::now();
        let tree = parser.parse(false);
        if let Some(_tree) = tree {
            println!("pure packrat: {:?}", Instant::now()-now);
            // println!("{}", tree);
        } else {
            println!("failure");
        }

        let lex = Lexer::new(input.clone());
        let mut parser = Parser::new(lex);
        let now = Instant::now();
        let tree = parser.parse(true);
        if let Some(_tree) = tree {
            println!("with pratt: {:?}", Instant::now()-now);
            // println!("{}", tree);
        } else {
            println!("failure");
        }
        
    }
}

#[test]
fn benchmark() {
    use std::time::Duration;
    let mut n = 1;
    println!("Pure Packrat\nNumber of Lines,Time to Parse");
    loop {
        let input = "let a: bool = !((~(1 + 1) ^ ((1 * 1 + 1 / 1 ) >> 3)) == ((8 & 4 / (16 | 16)) & 255)) && ~((8*8)>>8) > 256 * ((8 + 8)>>12) + 64;";
        let lex = Lexer::new(input.to_string().repeat(n));
        let mut parser = Parser::new(lex);
        let now = Instant::now();
        let tree = parser.parse(false);
        let t = Instant::now()-now;
        println!("{},{:?}", n, t);
        assert!(tree.is_some());
        if t > Duration::new(600, 0) {
            break
        }
        n *= 2;
    }

    n = 1;
    println!("Pratt\nNumber of Lines,Time to Parse");
    loop {
        let input = "let a: bool = !((~(1 + 1) ^ ((1 * 1 + 1 / 1 ) >> 3)) == ((8 & 4 / (16 | 16)) & 255)) && ~((8*8)>>8) > 256 * ((8 + 8)>>12) + 64;";
        let lex = Lexer::new(input.to_string().repeat(n));
        let mut parser = Parser::new(lex);
        let now = Instant::now();
        let tree = parser.parse(true);
        let t = Instant::now()-now;
        println!("{},{:?}", n, t);
        assert!(tree.is_some());
        if t > Duration::new(600, 0) {
            break
        }
        n *= 2;
    }
}


