#[derive(Clone, Debug)]
pub enum Token{ 
    // UNARY OPERATORS
    NEGATE,     // !BOOL
    AND,        // BOOL & BOOL
    OR,         // BOOL | BOOL
    XOR,        // BOOL ^ BOOL

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
    WHITESPACE(String),
    ID(String),
    NUMBER(String),
    STRING(String),

    // ERROR and EOF
    ERROR(String),
}

