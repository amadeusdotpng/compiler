#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Token{ 
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
    WHITESPACE(String),
    ID(String),
    NUMBER(String),
    STRING(String),

    // ERROR and EOF
    ERROR(String),
}

