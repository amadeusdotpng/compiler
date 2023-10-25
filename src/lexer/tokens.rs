use crate::packrat_parser::node::{Node, NodeType};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[allow(non_camel_case_types)]
pub enum TokenKind { 
    ASSIGN,     // =

    // BOOLEAN OPERATORS
    BOOL_NOT,        // !BOOL
    BOOL_AND,        // BOOL && BOOL
    BOOL_OR,         // BOOL || BOOL

    // BITWISE OPERATORS
    BIT_NOT,        // ~NUM
    BIT_AND,        // NUM & NUM
    BIT_OR,         // NUM | NUM
    BIT_XOR,        // NUM ^ NUM
    BIT_LEFT,       // NUM << NUM
    BIT_RIGHT,      // NUM >> NUM

    // ARITHMETIC OPERATORS
    PLUS,       // NUM + NUM
    MINUS,      // NUM - NUM
    MULTIPLY,   // NUM * NUM
    DIVIDE,     // NUM / NUM
    MODULUS,    // NUM % NUM

    // COMPARISONS
    EQ,         // ==
    NE,         // !=
    GT,         // >
    GE,         // >=
    LT,         // <
    LE,         // <=

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
    ELIF,
    ELSE,
    WHILE,
    FOR,
    DEF,
    
    // BOOL
    TRUE,
    FALSE,
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
