use crate::lexer::tokens::Token;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeType {
    Atom(Token),
    Cons(NodeKind),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NodeKind {
    Prog,
    Block,
    BlockExpr,
    Statements,
    Statement,
    Assignement,
    Expression,
    IfStmt,
    ElifStmt,
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
}

#[derive(Clone, Debug)]
pub struct Node {
    kind: NodeType,
    children: Option<Vec<Node>>,
}


impl Node {
    pub fn new(kind: NodeType, children: Option<Vec<Node>>) -> Node {
        Node {
            kind,
            children
        }
    }
}
