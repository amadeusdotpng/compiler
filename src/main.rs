mod lexer;
use lexer::{tokens::Token, lex::Lexer};
fn main() {
    let mut lex = Lexer::new(String::from("for;int;this_is_an_identifier;....;1234"));
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
