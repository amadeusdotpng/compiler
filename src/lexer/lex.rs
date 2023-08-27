use crate::lexer::tokens::Token;
use regex::Regex;

pub struct Lexer {
    input: String,
    curr_position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(s: String) -> Lexer {
//        let vec_char: Vec<char> = s.chars().collect();
        Lexer {
            input: s,
            curr_position: 0,
            read_position: 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        let mut curr_token = Token::ERROR(String::new());

        if self.read_position > self.input.len() {
            return Token::EOF;
        }

        loop {
            if self.read_position > self.input.len() {
                return curr_token;
            }

            let substring = &self.input[self.curr_position..self.read_position];
            let temp_token = match_string(substring);
//            println!("{:?}", temp_token);
            if let Token::ERROR(_) = temp_token {
                if let Token::ERROR(_) = curr_token {
                    self.read_position += 1;
                    self.curr_position = self.read_position-1;
                    return temp_token;
                } else {
                    self.curr_position = self.read_position-1;
                    return curr_token;
                }
            } else {
                self.read_position += 1;
                curr_token = temp_token;
            }
        }
    }
}

fn match_string(sub: &str) -> Token {
    return match sub {
        r"!" => Token::NEGATE,
        r"&" => Token::AND,
        r"|" => Token::OR,
        r"^" => Token::XOR,

        r"=" => Token::ASSIGN,
        r"+" => Token::PLUS,
        r"-" => Token::MINUS,
        r"*" => Token::MULTIPLY,
        r"/" => Token::DIVIDE,

        r"==" => Token::EQ,
        r"!=" => Token::NE,
        r">" => Token::GT,
        r">=" => Token::GQ,
        r"<" => Token::LT,
        r"<=" => Token::LQ,

        r"(" => Token::LPAREN,
        r")" => Token::RPAREN,
        r"{" => Token::LCURLY,
        r"}" => Token::RCURLY,
        r"[" => Token::LBRACE,
        r"]" => Token::RBRACE,
        r":" => Token::COLON,
        r";" => Token::SEMICOLON,
        r"," => Token::COMMA,

        r"int" => Token::INT,
        r"bool" => Token::BOOL,
        r"while" => Token::WHILE,
        r"if" => Token::IF,
        r"else" => Token::ELSE,
        r"return" => Token::RETURN,
        _ => regex_match(sub),
    };
}

fn regex_match(sub: &str) -> Token {
    let re_whitespace = Regex::new(r"^\s+$").unwrap();
    let re_number = Regex::new(r"^\d+$").unwrap();
    let re_identifier = Regex::new(r"^[a-zA-Z_]+$").unwrap();
    
    if re_whitespace.is_match(sub)      { Token::WHITESPACE(String::from(sub)) }
    else if re_number.is_match(sub)     { Token::NUMBER(String::from(sub))     }
    else if re_identifier.is_match(sub) { Token::ID(String::from(sub))         }
    else { Token::ERROR(String::from(sub)) }
}
