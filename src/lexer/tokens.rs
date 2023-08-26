#[derive(Clone, Debug)]
pub enum Token{ 
    // UNARY OPERATORS
    NEGATE,  // !BOOL
    AND,        // BOOL & BOOL
    OR,         // BOOL | BOOL

    // BINARY OPERATORS
    ASSIGN,     // =
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
    INT,
    BOOL,
    WHILE,
    IF,
    ELSE,
    RETURN,

    // STUFF
    WHITESPACE(String),
    NUMBER(String),
    ID(String),

    // ERROR and EOF
    ERROR(String),
    EOF
}

