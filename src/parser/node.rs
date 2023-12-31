use super::parser::Parser;
use crate::lexer::tokens::{Token, TokenKind};
use std::fmt;
use super::pratt::parse_expression;

#[derive(Clone, Debug)]
pub struct Node {
    kind: NodeType,
    children: Option<Vec<Node>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            NodeType::Cons(kind) => {
                let children = self.children.as_ref().expect("Node of Cons type does has None children")
                    .into_iter()
                    .map(|child| child.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "({:?} {})", kind, children)
            },
            NodeType::Atom(kind) => {
                if let Some(children) = &self.children {
                    let children = children
                        .into_iter()
                        .map(|child| child.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    write!(f, "({} {})", kind, children)
                } else {
                    write!(f, "{}", kind)
                }
            }
        }
    }
}

impl Node {
    pub fn new(kind: NodeType, children: Option<Vec<Node>>) -> Node {
        Node { kind, children }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeType {
    Atom(Token),
    Cons(NodeKind),
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum NodeKind {
    Prog,
    Block,
    BlockExpr,
    Statements,
    Statement,
    Declaration,
    Assignment,
    Expression,
    IfStmt,
    ElseStmt,
    WhileStmt,
    LogicOr,
    LogicAnd,
    LogicNot,
    Comparison,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    BitwiseShift,
    Sum,
    Term,
    Factor,
    Primary,
    DataType,
}

/* If NodeKind is left recursive */
impl Into<bool> for NodeKind {
    fn into(self) -> bool {
        match self {
            NodeKind::Statements
            | NodeKind::LogicOr
            | NodeKind::LogicAnd
            | NodeKind::BitwiseOr
            | NodeKind::BitwiseXor
            | NodeKind::BitwiseAnd
            | NodeKind::BitwiseShift
            | NodeKind::Sum
            | NodeKind::Term => true,
            _ => false,
        }
    }
}

impl NodeKind {
    pub fn parse(self, parser: &mut Parser) -> Option<Node> {
        match self {
            NodeKind::Prog => parser.memoize(prog, self),
            NodeKind::Block => parser.memoize(block, self),
            NodeKind::BlockExpr => parser.memoize(block_expr, self),
            NodeKind::Statements => parser.memoize(statements, self),
            NodeKind::Statement => parser.memoize(statement, self),
            NodeKind::Declaration => parser.memoize(declaration, self),
            NodeKind::Assignment => parser.memoize(assignment, self),
            NodeKind::Expression => parser.memoize(expression, self),
            NodeKind::IfStmt => parser.memoize(if_stmt, self),
            NodeKind::ElseStmt => parser.memoize(else_stmt, self),
            NodeKind::WhileStmt => parser.memoize(while_stmt, self),
            NodeKind::LogicOr => parser.memoize(logic_or, self),
            NodeKind::LogicAnd => parser.memoize(logic_and, self),
            NodeKind::LogicNot => parser.memoize(logic_not, self),
            NodeKind::Comparison => parser.memoize(comparison, self),
            NodeKind::BitwiseOr => parser.memoize(bitwise_or, self),
            NodeKind::BitwiseXor => parser.memoize(bitwise_xor, self),
            NodeKind::BitwiseAnd => parser.memoize(bitwise_and, self),
            NodeKind::BitwiseShift => parser.memoize(bitwise_shift, self),
            NodeKind::Sum => parser.memoize(sum, self),
            NodeKind::Term => parser.memoize(term, self),
            NodeKind::Factor => parser.memoize(factor, self),
            NodeKind::Primary => parser.memoize(primary, self),
            NodeKind::DataType => parser.memoize(datatype, self),
        }
    }
}

enum Rules {
    Terminal(TokenKind),
    NonTerminal(NodeKind),
}

fn parse_productions(
    parser: &mut Parser,
    productions: &[Vec<Rules>],
    kind: NodeType,
) -> Option<Node> {
    let start = parser.mark();
    for prod in productions {
        parser.reset(start);
        if let Some(mut children) = production(parser, &prod) {
            if children.len() == 1 {return children.pop()} // Makes parse trees a lot smaller
            return Some(Node::new(kind, Some(children)));
        }
    }
    parser.reset(start);
    return None;
}

fn production(parser: &mut Parser, rules: &Vec<Rules>) -> Option<Vec<Node>> {
    let mut children: Vec<Node> = vec![];
    for rule in rules {
        let child: Option<Node> = match rule {
            Rules::Terminal(kind) => {
                if let Some(child) = parser.expect(*kind) {
                    Some(Node::new(NodeType::Atom(child), None))
                } else {
                    None
                }
            }
            Rules::NonTerminal(kind) => (*kind).parse(parser),
        };
        if let Some(child) = child {
            children.push(child);
        } else {
            return None;
        }
    }
    Some(children)
}

fn prog(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Prog);
    let productions = [vec![
        Rules::NonTerminal(NodeKind::Statements),
        Rules::Terminal(TokenKind::EOF),
    ]];
    return parse_productions(parser, &productions, kind);
}

/* Blocks, Statements, and Expressions */
fn block(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Block);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::LCURLY),
            Rules::NonTerminal(NodeKind::Statements),
            Rules::Terminal(TokenKind::RCURLY),
        ],
        vec![Rules::NonTerminal(NodeKind::BlockExpr)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn block_expr(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::BlockExpr);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::LCURLY),
            Rules::NonTerminal(NodeKind::Statements),
            Rules::NonTerminal(NodeKind::Expression),
            Rules::Terminal(TokenKind::RCURLY),
        ],
        vec![
            Rules::Terminal(TokenKind::LCURLY),
            Rules::NonTerminal(NodeKind::Expression),
            Rules::Terminal(TokenKind::RCURLY),
        ],
    ];
    return parse_productions(parser, &productions, kind);
}

fn statements(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Statements);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::Statement),
            Rules::NonTerminal(NodeKind::Statements),
        ],
        vec![Rules::NonTerminal(NodeKind::Statement)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn statement(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Statement);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::Declaration),
            Rules::Terminal(TokenKind::SEMICOLON),
        ],
        vec![
            Rules::NonTerminal(NodeKind::Assignment),
            Rules::Terminal(TokenKind::SEMICOLON),
        ],
        vec![Rules::NonTerminal(NodeKind::IfStmt)],
        vec![Rules::NonTerminal(NodeKind::WhileStmt)],
        vec![Rules::NonTerminal(NodeKind::Block)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn expression(parser: &mut Parser) -> Option<Node> {
    if parser.pratt {
        Some(parse_expression(parser))
    } else {
        let kind = NodeType::Cons(NodeKind::Expression);
        let productions = [
            vec![Rules::NonTerminal(NodeKind::LogicOr)],
            vec![Rules::NonTerminal(NodeKind::IfStmt)],
        ];
        parse_productions(parser, &productions, kind)
    }
}

fn declaration(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Declaration);
    let productions = [vec![
        Rules::Terminal(TokenKind::LET),
        Rules::Terminal(TokenKind::ID),
        Rules::Terminal(TokenKind::COLON),
        Rules::NonTerminal(NodeKind::DataType),
        Rules::Terminal(TokenKind::ASSIGN),
        Rules::NonTerminal(NodeKind::Expression),
    ]];
    return parse_productions(parser, &productions, kind);
}

fn assignment(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Assignment);
    let productions = [vec![
        Rules::Terminal(TokenKind::ID),
        Rules::Terminal(TokenKind::ASSIGN),
        Rules::NonTerminal(NodeKind::Expression),
    ]];
    return parse_productions(parser, &productions, kind);
}

/* If Statements */
fn if_stmt(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::IfStmt);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::IF),
            Rules::NonTerminal(NodeKind::Expression),
            Rules::NonTerminal(NodeKind::Block),
            Rules::NonTerminal(NodeKind::ElseStmt),
        ],
        vec![
            Rules::Terminal(TokenKind::IF),
            Rules::NonTerminal(NodeKind::Expression),
            Rules::NonTerminal(NodeKind::Block),
        ],
    ];
    return parse_productions(parser, &productions, kind);
}

fn else_stmt(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::ElseStmt);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::ELSE),
            Rules::NonTerminal(NodeKind::Block),
        ],
        vec![
            Rules::Terminal(TokenKind::ELSE),
            Rules::NonTerminal(NodeKind::IfStmt),
        ]
    ];
    return parse_productions(parser, &productions, kind);
}

/* While Statment */
fn while_stmt(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::WhileStmt);
    let productions = [vec![
        Rules::Terminal(TokenKind::WHILE),
        Rules::NonTerminal(NodeKind::Expression),
        Rules::NonTerminal(NodeKind::Block),
    ]];
    return parse_productions(parser, &productions, kind);
}

/* Logic Operators */
fn logic_or(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::LogicOr);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::LogicOr),
            Rules::Terminal(TokenKind::BOOL_OR),
            Rules::NonTerminal(NodeKind::LogicAnd),
        ],
        vec![Rules::NonTerminal(NodeKind::LogicAnd)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn logic_and(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::LogicAnd);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::LogicAnd),
            Rules::Terminal(TokenKind::BOOL_AND),
            Rules::NonTerminal(NodeKind::LogicNot),
        ],
        vec![Rules::NonTerminal(NodeKind::LogicNot)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn logic_not(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::LogicNot);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::BOOL_NOT),
            Rules::NonTerminal(NodeKind::LogicNot),
        ],
        vec![Rules::NonTerminal(NodeKind::Comparison)],
    ];
    return parse_productions(parser, &productions, kind);
}
/* Comparison Operators */
fn comparison(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Comparison);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::EQ),
            Rules::NonTerminal(NodeKind::BitwiseOr),
        ],
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::NE),
            Rules::NonTerminal(NodeKind::BitwiseOr),
        ],
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::GE),
            Rules::NonTerminal(NodeKind::BitwiseOr),
        ],
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::GT),
            Rules::NonTerminal(NodeKind::BitwiseOr),
        ],
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::LE),
            Rules::NonTerminal(NodeKind::BitwiseOr),
        ],
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::LT),
            Rules::NonTerminal(NodeKind::BitwiseOr),
        ],
        vec![Rules::NonTerminal(NodeKind::BitwiseOr)],
    ];
    return parse_productions(parser, &productions, kind);
}
/* Bitwise Operators */
fn bitwise_or(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::BitwiseOr);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::BitwiseOr),
            Rules::Terminal(TokenKind::BIT_OR),
            Rules::NonTerminal(NodeKind::BitwiseXor),
        ],
        vec![Rules::NonTerminal(NodeKind::BitwiseXor)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn bitwise_xor(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::BitwiseXor);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::BitwiseXor),
            Rules::Terminal(TokenKind::BIT_XOR),
            Rules::NonTerminal(NodeKind::BitwiseAnd),
        ],
        vec![Rules::NonTerminal(NodeKind::BitwiseAnd)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn bitwise_and(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::BitwiseAnd);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::BitwiseAnd),
            Rules::Terminal(TokenKind::BIT_AND),
            Rules::NonTerminal(NodeKind::BitwiseShift),
        ],
        vec![Rules::NonTerminal(NodeKind::BitwiseShift)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn bitwise_shift(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::BitwiseShift);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::BitwiseShift),
            Rules::Terminal(TokenKind::BIT_LEFT),
            Rules::NonTerminal(NodeKind::Sum),
        ],
        vec![
            Rules::NonTerminal(NodeKind::BitwiseShift),
            Rules::Terminal(TokenKind::BIT_RIGHT),
            Rules::NonTerminal(NodeKind::Sum),
        ],
        vec![Rules::NonTerminal(NodeKind::Sum)],
    ];
    return parse_productions(parser, &productions, kind);
}

/* Arithmetic Operators */
fn sum(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Sum);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::Sum),
            Rules::Terminal(TokenKind::PLUS),
            Rules::NonTerminal(NodeKind::Term),
        ],
        vec![
            Rules::NonTerminal(NodeKind::Sum),
            Rules::Terminal(TokenKind::MINUS),
            Rules::NonTerminal(NodeKind::Term),
        ],
        vec![Rules::NonTerminal(NodeKind::Term)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn term(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Term);
    let productions = [
        vec![
            Rules::NonTerminal(NodeKind::Term),
            Rules::Terminal(TokenKind::MULTIPLY),
            Rules::NonTerminal(NodeKind::Factor),
        ],
        vec![
            Rules::NonTerminal(NodeKind::Term),
            Rules::Terminal(TokenKind::DIVIDE),
            Rules::NonTerminal(NodeKind::Factor),
        ],
        vec![
            Rules::NonTerminal(NodeKind::Term),
            Rules::Terminal(TokenKind::MODULUS),
            Rules::NonTerminal(NodeKind::Factor),
        ],
        vec![Rules::NonTerminal(NodeKind::Factor)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn factor(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Factor);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::PLUS),
            Rules::NonTerminal(NodeKind::Factor),
        ],
        vec![
            Rules::Terminal(TokenKind::MINUS),
            Rules::NonTerminal(NodeKind::Factor),
        ],
        vec![
            Rules::Terminal(TokenKind::BIT_NOT),
            Rules::NonTerminal(NodeKind::Factor),
        ],
        vec![Rules::NonTerminal(NodeKind::Primary)],
    ];
    return parse_productions(parser, &productions, kind);
}

/* Atoms */
fn primary(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::Primary);
    let productions = [
        vec![
            Rules::Terminal(TokenKind::LPAREN),
            Rules::NonTerminal(NodeKind::Expression),
            Rules::Terminal(TokenKind::RPAREN),
        ],
        vec![Rules::NonTerminal(NodeKind::BlockExpr)],
        vec![Rules::NonTerminal(NodeKind::IfStmt)],
        vec![Rules::Terminal(TokenKind::NUMBER)],
        vec![Rules::Terminal(TokenKind::STRING)],
        vec![Rules::Terminal(TokenKind::ID)],
        vec![Rules::Terminal(TokenKind::TRUE)],
        vec![Rules::Terminal(TokenKind::FALSE)],
    ];
    return parse_productions(parser, &productions, kind);
}

fn datatype(parser: &mut Parser) -> Option<Node> {
    let kind = NodeType::Cons(NodeKind::DataType);
    let productions = [
        vec![Rules::Terminal(TokenKind::INT)],
        vec![Rules::Terminal(TokenKind::BOOL)],
        vec![Rules::Terminal(TokenKind::STR)],
    ];
    return parse_productions(parser, &productions, kind);
}
