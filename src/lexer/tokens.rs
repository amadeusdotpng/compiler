use crate::packrat_parser::node::{Node, NodeType};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum TokenKind { 
    ASSIGN,     // =

    // BOOLEAN OPERATORS
    BOOL_NOT,        // !BOOL
    BOOL_AND,        // BOOL && BOOL
    BOOL_OR,         // BOOL || BOOL

    // BINARY OPERATORS
    BIN_NOT,        // ~NUM
    BIN_AND,        // NUM & NUM
    BIN_OR,         // NUM | NUM
    BIN_XOR,        // NUM ^ NUM
    BIN_LEFT,       // NUM << NUM
    BIN_RIGHT,      // NUM >> NUM

    // ARITHMETIC OPERATORS
    PLUS,       // NUM + NUM
    MINUS,      // NUM - NUM
    MULTIPLY,   // NUM * NUM
    DIVIDE,     // NUM / NUM

    // COMPARISONS
    EQ,         // ==
    NE,         // !=
    GT,         // >
    GQ,         // >=
    LT,         // <
    LQ,         // <=

    // PUNCTUATION
    LPAREN,     // (
    RPAREN,     // )
    LCURLY,     // {
    RCURLY,     // }
    LBRACE,     // [
    RBRACE,     // ]
    COLON,      // :
    SEMICOLON,  // ;
    COMMA,      // ,

    // KEYWORDS
    // TYPES
    INT,
    BOOL,
    STR,

    // STATEMENTS
    IF,
    ELSE,
    WHILE,
    FOR,
    DEF,
//    RETURN,

    // STUFF
    WHITESPACE,
    ID,
    NUMBER,
    STRING,

    // ERROR and EOF
    ERROR,
    EOF,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Option<String>,
    pub position: (usize, usize),
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: Option<&str>, position: (usize, usize)) -> Token {
        Token {
            kind,
            lexeme: match lexeme {
                Some(str) => Some(str.into()),
                None => None,
            },
            position,
        }
    }
}

impl Into<Node> for Token {
    fn into(self) -> Node {
        Node::new(NodeType::Atom(self), None)
    }
}
