use crate::lexer::{tokens::{Token, TokenKind}, lex::Lexer};
use crate::packrat_parser::node::{NodeKind, NodeType, Node};
use std::collections::HashMap;

pub struct PackratParser {
    lex: Lexer,
    cache: HashMap<(NodeKind, (usize, usize)), (Option<Node>, (usize, usize))>
}

impl PackratParser {
    pub fn new(lex: Lexer) -> PackratParser {
        PackratParser {
            lex,
            cache: HashMap::new(),
        }
    }

    pub fn mark(&self) -> (usize, usize) {
        self.lex.mark()
    }

    pub fn reset(&mut self, location: (usize, usize)) {
        self.lex.reset(location)
    }

    pub fn expect(&mut self, tok: TokenKind) -> Option<Token> {
        if tok == self.lex.peek().kind {
            Some(self.lex.next())
        } else{
            None
        }
    }

    fn memoize(&mut self, f: fn(&mut PackratParser) -> Option<Node>, kind: NodeKind) -> Option<Node> {
        let start_position = self.mark();
        let key = (kind.clone(), start_position);

        if let Some((node, end_position)) = self.cache.get(&key) {
            let node = node.clone();
            self.reset(end_position.clone());
            return node
        } else if kind.clone().into() {
            let (mut last_node, mut last_position) = (None, start_position);
            self.cache.insert(key.clone(), (None, start_position));
            loop {
                self.reset(start_position);

                let node = f(self);
                let end_position = self.mark();

                if end_position.1 <= last_position.1 {
                    self.reset(last_position);
                    return last_node;
                }

                (last_node, last_position) = (node, end_position);
                self.cache.insert(key.clone(), (last_node.clone(), last_position));
            }
        } else {
            let node = f(self);
            let end_position = self.mark();
            self.cache.insert(key.clone(), (node.clone(), end_position));
            return node
        }
    }

    pub fn parse(&mut self) -> Option<Node> {
        let rule0 = self.memoize(statements, NodeKind::Statements);
        let rule1 = {
            let tok = self.expect(TokenKind::EOF);
            if let Some(tok) = tok {
                Some(Node::new(NodeType::Atom(tok), None))
            } else {
                None
            }
        };

        if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
            Some(Node::new(NodeType::Cons(NodeKind::Prog), Some(vec![rule0, rule1])))
        } else {
            None
        }
    }
}


/* blocks and statements and expressions */
fn block(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Block);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![block_0, block_1];

    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn block_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::LCURLY);
    let rule1 = parser.memoize(statements, NodeKind::Statements);
    let rule2 = parser.expect(TokenKind::RCURLY);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0.into(), rule1, rule2.into()])
    }
    return None
}

fn block_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(block_expr, NodeKind::BlockExpr);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn block_expr(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::BlockExpr);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![block_expr_0, block_expr_1];

    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn block_expr_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::LCURLY);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.expect(TokenKind::RCURLY);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0.into(), rule1, rule2.into()])
    }
    return None
}

fn block_expr_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::LCURLY);
    let rule1 = parser.memoize(statements, NodeKind::Expression);
    let rule2 = parser.memoize(expression, NodeKind::Expression);
    let rule3 = parser.expect(TokenKind::RCURLY);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3)) = (rule0, rule1, rule2, rule3) {
        return Some(vec![rule0.into(), rule1, rule2, rule3.into()])
    }
    return None
}


fn statements(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Statements);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![statements_0, statements_1];

    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn statements_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(statements, NodeKind::Statements);
    let rule1 = parser.memoize(statement, NodeKind::Statement);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        return Some(vec![rule0, rule1])
    }
    None
}

fn statements_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(statement, NodeKind::Statement);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    None
}


fn statement(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Statement);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![statement_0, statement_1, statement_2, statement_3];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn statement_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(assignment, NodeKind::Assignement);
    let rule1 = parser.expect(TokenKind::SEMICOLON);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        return Some(vec![rule0, rule1.into()])
    } 
    None
}

fn statement_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(if_stmt, NodeKind::IfStmt);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    } 
    None
}

fn statement_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(while_stmt, NodeKind::WhileStmt);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    } 
    None
}

fn statement_3(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(block, NodeKind::Block);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    } 
    None
}

fn assignment(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Assignement);
    let rule0 = parser.expect(TokenKind::ID);
    let rule1 = parser.expect(TokenKind::COLON);
    let rule2 = parser.memoize(datatype, NodeKind::DataType);
    let rule3 = parser.expect(TokenKind::ASSIGN);
    let rule4 = parser.memoize(expression, NodeKind::Expression);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3), Some(rule4)) = (rule0, rule1, rule2, rule3, rule4) {
        let children = vec![rule0.into(), rule1.into(), rule2, rule3.into(), rule4];
        return Some(Node::new(kind, Some(children)))
    } 
    
    parser.reset(start);
    return None
}

fn expression(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Expression);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![expression_0, expression_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn expression_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(logic_or, NodeKind::LogicOr);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    } 
    None
}

fn expression_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(if_stmt, NodeKind::IfStmt);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    } 
    None
}


/* if statements */
fn if_stmt(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::IfStmt);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![if_stmt_0, if_stmt_1, if_stmt_2];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn if_stmt_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::IF);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);
    let rule3 = parser.memoize(elif_stmt, NodeKind::ElifStmt);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3)) = (rule0, rule1, rule2, rule3) {
        return Some(vec![rule0.into(), rule1, rule2, rule3.into()])
    }
    return None
}

fn if_stmt_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::IF);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);
    let rule3 = parser.memoize(else_stmt, NodeKind::ElseStmt);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3)) = (rule0, rule1, rule2, rule3) {
        return Some(vec![rule0.into(), rule1, rule2, rule3.into()])
    }
    return None
}

fn if_stmt_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::IF);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0.into(), rule1, rule2])
    }
    return None
}

fn elif_stmt(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::ElifStmt);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![elif_stmt_0, elif_stmt_1, elif_stmt_2];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn elif_stmt_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::ELIF);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);
    let rule3 = parser.memoize(elif_stmt, NodeKind::ElifStmt);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3)) = (rule0, rule1, rule2, rule3) {
        return Some(vec![rule0.into(), rule1, rule2, rule3.into()])
    }
    return None
}

fn elif_stmt_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::ELIF);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);
    let rule3 = parser.memoize(else_stmt, NodeKind::ElseStmt);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3)) = (rule0, rule1, rule2, rule3) {
        return Some(vec![rule0.into(), rule1, rule2, rule3.into()])
    }
    return None
}

fn elif_stmt_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::ELIF);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0.into(), rule1, rule2])
    }
    return None
}

fn else_stmt(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::ElseStmt);
    let rule0 = parser.expect(TokenKind::ELSE);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        let children = vec![rule0.into(), rule1, rule2];
        return Some(Node::new(kind, Some(children)))
    }
    
    parser.reset(start);
    return None
}


/* while statement */
fn while_stmt(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::WhileStmt);
    let rule0 = parser.expect(TokenKind::WHILE);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.memoize(block, NodeKind::Block);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        let children = vec![rule0.into(), rule1, rule2];
        return Some(Node::new(kind, Some(children)))
    }
    
    parser.reset(start);
    return None
}

/* logic operators */
fn logic_or(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::LogicOr);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![logic_or_0, logic_or_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn logic_or_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(logic_or, NodeKind::LogicOr);
    let rule1 = parser.expect(TokenKind::BOOL_OR);
    let rule2 = parser.memoize(logic_and, NodeKind::LogicAnd);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn logic_or_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(logic_and, NodeKind::LogicAnd);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0]);
    }
    return None
}

fn logic_and(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::LogicAnd);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![logic_and_0, logic_and_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn logic_and_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(logic_and, NodeKind::LogicAnd);
    let rule1 = parser.expect(TokenKind::BOOL_AND);
    let rule2 = parser.memoize(logic_not, NodeKind::LogicNot);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn logic_and_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(logic_not, NodeKind::LogicNot);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0]);
    }
    return None
}

fn logic_not(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::LogicNot);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![logic_not_0, logic_not_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn logic_not_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::BOOL_NOT);
    let rule1 = parser.memoize(logic_not, NodeKind::LogicNot);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        return Some(vec![rule0.into(), rule1])
    }
    return None
}

fn logic_not_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(comparison, NodeKind::Comparison);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0]);
    }
    return None
}


fn comparison(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Comparison);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![comparison_0, comparison_1, comparison_2, comparison_3, comparison_4, comparison_5, comparison_6];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn comparison_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::EQ);
    let rule2 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn comparison_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::NE);
    let rule2 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn comparison_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::LQ);
    let rule2 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn comparison_3(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::LT);
    let rule2 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn comparison_4(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::GQ);
    let rule2 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn comparison_5(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::GT);
    let rule2 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn comparison_6(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn bitwise_or(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::BitwiseOr);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![bitwise_or_0, bitwise_or_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn bitwise_or_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_or, NodeKind::BitwiseOr);
    let rule1 = parser.expect(TokenKind::BIN_OR);
    let rule2 = parser.memoize(bitwise_xor, NodeKind::BitwiseXor);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn bitwise_or_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_xor, NodeKind::BitwiseXor);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn bitwise_xor(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::BitwiseXor);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![bitwise_xor_0, bitwise_xor_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn bitwise_xor_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_xor, NodeKind::BitwiseXor);
    let rule1 = parser.expect(TokenKind::BIN_XOR);
    let rule2 = parser.memoize(bitwise_and, NodeKind::BitwiseAnd);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn bitwise_xor_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_and, NodeKind::BitwiseAnd);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn bitwise_and(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::BitwiseAnd);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![bitwise_and_0, bitwise_and_1];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn bitwise_and_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_and, NodeKind::BitwiseAnd);
    let rule1 = parser.expect(TokenKind::BIN_AND);
    let rule2 = parser.memoize(bitwise_shift, NodeKind::BitwiseShift);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn bitwise_and_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_shift, NodeKind::BitwiseShift);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn bitwise_shift(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::BitwiseShift);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![bitwise_shift_0, bitwise_shift_1, bitwise_shift_2];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn bitwise_shift_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_shift, NodeKind::BitwiseShift);
    let rule1 = parser.expect(TokenKind::BIN_LEFT);
    let rule2 = parser.memoize(sum, NodeKind::Sum);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn bitwise_shift_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(bitwise_shift, NodeKind::BitwiseShift);
    let rule1 = parser.expect(TokenKind::BIN_RIGHT);
    let rule2 = parser.memoize(sum, NodeKind::Sum);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn bitwise_shift_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(sum, NodeKind::Sum);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn sum(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Sum);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![sum_0, sum_1, sum_2];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn sum_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(sum, NodeKind::Sum);
    let rule1 = parser.expect(TokenKind::PLUS);
    let rule2 = parser.memoize(term, NodeKind::Term);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn sum_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(sum, NodeKind::Sum);
    let rule1 = parser.expect(TokenKind::MINUS);
    let rule2 = parser.memoize(term, NodeKind::Term);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn sum_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(term, NodeKind::Term);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn term(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Term);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![term_0, term_1, term_2, term_3];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn term_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(term, NodeKind::Term);
    let rule1 = parser.expect(TokenKind::MULTIPLY);
    let rule2 = parser.memoize(factor, NodeKind::Factor);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn term_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(term, NodeKind::Term);
    let rule1 = parser.expect(TokenKind::DIVIDE);
    let rule2 = parser.memoize(factor, NodeKind::Factor);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn term_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(term, NodeKind::Term);
    let rule1 = parser.expect(TokenKind::MODULUS);
    let rule2 = parser.memoize(factor, NodeKind::Factor);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2) {
        return Some(vec![rule0, rule1.into(), rule2])
    }
    return None
}

fn term_3(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(factor, NodeKind::Factor);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn factor(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Factor);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![factor_0, factor_1, factor_2, factor_3];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn factor_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::PLUS);
    let rule1 = parser.memoize(factor, NodeKind::Factor);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        return Some(vec![rule0.into(), rule1])
    }
    return None
}

fn factor_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::MINUS);
    let rule1 = parser.memoize(factor, NodeKind::Factor);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        return Some(vec![rule0.into(), rule1])
    }
    return None
}

fn factor_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::BIN_NOT);
    let rule1 = parser.memoize(factor, NodeKind::Factor);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        return Some(vec![rule0.into(), rule1])
    }
    return None
}

fn factor_3(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(primary, NodeKind::Primary);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn primary(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Factor);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![primary_0, primary_1, primary_2, primary_3, primary_4, primary_5];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn primary_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::LPAREN);
    let rule1 = parser.memoize(expression, NodeKind::Expression);
    let rule2 = parser.expect(TokenKind::RPAREN);

    if let (Some(rule0), Some(rule1), Some(rule2)) = (rule0, rule1, rule2)  {
        return Some(vec![rule0.into(), rule1, rule2.into()])
    }
    return None
}

fn primary_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.memoize(block_expr, NodeKind::BlockExpr);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0])
    }
    return None
}

fn primary_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::NUMBER);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0.into()])
    }
    return None
}

fn primary_3(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::TRUE);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0.into()])
    }
    return None
}

fn primary_4(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::FALSE);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0.into()])
    }
    return None
}

fn primary_5(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::ID);

    if let Some(rule0) = rule0 {
        return Some(vec![rule0.into()])
    }
    return None
}

fn datatype(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::DataType);
    let productions: Vec<fn(&mut PackratParser) -> Option<Vec<Node>>> 
        = vec![datatype_0, datatype_1, datatype_2];
    for production in productions {
        parser.reset(start);
        if let Some(children) = production(parser) {
            return Some(Node::new(kind, Some(children)));
        }

    }
    parser.reset(start);
    return None
}

fn datatype_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::INT);

    if let Some(rule0) = rule0  {
        return Some(vec![rule0.into()])
    }
    return None
}

fn datatype_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::BOOL);

    if let Some(rule0) = rule0  {
        return Some(vec![rule0.into()])
    }
    return None
}

fn datatype_2(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.expect(TokenKind::STR);

    if let Some(rule0) = rule0  {
        return Some(vec![rule0.into()])
    }
    return None
}
