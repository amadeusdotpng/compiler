use crate::lexer::tokens::{Token, TokenKind};
use regex::Regex;

pub struct Lexer {
    input: String,
    position: usize,
    lookahead: usize,
}

impl Lexer {
    pub fn new(s: String) -> Lexer {
//        let vec_char: Vec<char> = s.chars().collect();
        Lexer {
            input: s,
            position: 0,
            lookahead: 1,
        }
    }

    pub fn mark(&self) -> (usize, usize) {
        (self.position, self.lookahead)
    }

    pub fn reset(&mut self, location: (usize, usize)) {
        (self.position, self.lookahead) = location;
    }

    pub fn next(&mut self) -> Token {
        let mut token_buffer = TokenKind::ERROR;
        let mut substring_buffer = "";

        if self.lookahead >= self.input.len() {
            return Token::new(TokenKind::EOF, None, self.mark())
        }

        loop {
            if self.lookahead > self.input.len() {
                return Token::new(token_buffer, Some(substring_buffer), self.mark())
            }

            let substring = &self.input[self.position..self.lookahead];
            let token = match_string(substring);

            if token == TokenKind::ERROR {
                if token_buffer == TokenKind::ERROR {
                    self.lookahead += 1;
                    self.position = self.lookahead-1;
                    return Token::new(token, Some(substring), self.mark());
                }
                else if token_buffer == TokenKind::WHITESPACE {
                    self.position = self.lookahead-1;
                } else {
                    self.position = self.lookahead-1;
                    return Token::new(token_buffer, Some(substring_buffer), self.mark());
                }
            } else {
                self.lookahead += 1;
                token_buffer = token;
                substring_buffer = substring;
            }
        }
    }

    pub fn peek(&mut self) -> Token {
        let pos = self.mark();
        let token = self.next();
        self.reset(pos);
        token
    }
}

fn match_string(sub: &str) -> TokenKind {
    return match sub {
        r"=" => TokenKind::ASSIGN,

        r"!"  => TokenKind::BOOL_NOT,
        r"&&" => TokenKind::BOOL_AND,
        r"||" => TokenKind::BOOL_OR,

        r"~"  => TokenKind::BIT_NOT,
        r"&"  => TokenKind::BIT_AND,
        r"|"  => TokenKind::BIT_OR,
        r"^"  => TokenKind::BIT_XOR,
        r"<<" => TokenKind::BIT_LEFT,
        r">>" => TokenKind::BIT_RIGHT,

        r"+" => TokenKind::PLUS,
        r"-" => TokenKind::MINUS,
        r"*" => TokenKind::MULTIPLY,
        r"/" => TokenKind::DIVIDE,
        r"%" => TokenKind::MODULUS,

        r"==" => TokenKind::EQ,
        r"!=" => TokenKind::NE,
        r">"  => TokenKind::GT,
        r">=" => TokenKind::GE,
        r"<"  => TokenKind::LT,
        r"<=" => TokenKind::LE,

        r"(" => TokenKind::LPAREN,
        r")" => TokenKind::RPAREN,
        r"{" => TokenKind::LCURLY,
        r"}" => TokenKind::RCURLY,
        r"[" => TokenKind::LBRACE,
        r"]" => TokenKind::RBRACE,
        r":" => TokenKind::COLON,
        r";" => TokenKind::SEMICOLON,
        r"," => TokenKind::COMMA,

        r"int" => TokenKind::INT,
        r"bool" => TokenKind::BOOL,
        r"str" => TokenKind::STR,

        r"if" => TokenKind::IF,
        r"elif" => TokenKind::ELIF,
        r"else" => TokenKind::ELSE,
        r"while" => TokenKind::WHILE,
        r"for" => TokenKind::FOR,
        r"def" => TokenKind::DEF,

        r"true" => TokenKind::TRUE,
        r"false" => TokenKind::FALSE,

//        r"return" => TokenKind::RETURN,
        _ => regex_match(sub),
    };
}

fn regex_match(sub: &str) -> TokenKind {
    let re_whitespace = Regex::new(r"^\s+$").unwrap();
    let re_identifier = Regex::new(r"^[a-zA-Z_]+$").unwrap();
    let re_number = Regex::new(r"^\d+$").unwrap();
    let re_string = Regex::new(r#"^"[a-zA-Z\d]+"$"#).unwrap();
    
    if re_whitespace.is_match(sub)      { TokenKind::WHITESPACE }
    else if re_identifier.is_match(sub) { TokenKind::ID     }
    else if re_number.is_match(sub)     { TokenKind::NUMBER }
    else if re_string.is_match(sub)     { TokenKind::STRING }
    else { TokenKind::ERROR }
}
