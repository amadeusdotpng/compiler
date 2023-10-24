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

    fn left_memoize(&mut self, f: fn(&mut PackratParser) -> Option<Node>, kind: NodeKind) -> Option<Node> {
        let start_position = self.mark();
        let key = (kind, start_position);

        if let Some((node, end_position)) = self.cache.get(&key) {
            let node = node.clone();
            self.reset(end_position.clone());
            return node
        } else {
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
        }
    }

    pub fn parse(&mut self) -> Option<Node> {
        let rule0 = self.left_memoize(statements, NodeKind::Statements);
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

fn statements(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let kind = NodeType::Cons(NodeKind::Statements);

    if let Some(production) = statements_0(parser) {
        return Some(Node::new(kind, Some(production)))
    }

    parser.reset(start);
    if let Some(production) = statements_1(parser) {
        return Some(Node::new(kind, Some(production)))
    }

    parser.reset(start);
    return None
}

fn statements_0(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = parser.left_memoize(statements, NodeKind::Statements);
    let rule1 = assignment(parser);

    if let (Some(rule0), Some(rule1)) = (rule0, rule1) {
        Some(vec![rule0, rule1])
    } else {
        None
    }
}

fn statements_1(parser: &mut PackratParser) -> Option<Vec<Node>> {
    let rule0 = assignment(parser);

    if let Some(rule0) = rule0 {
        Some(vec![rule0])
    } else {
        None
    }
}

fn assignment(parser: &mut PackratParser) -> Option<Node> {
    let start = parser.mark();
    let rule0 = parser.expect(TokenKind::ID);
    let rule1 = parser.expect(TokenKind::ASSIGN);
    let rule2 = parser.expect(TokenKind::NUMBER);
    let rule3 = parser.expect(TokenKind::SEMICOLON);
    let kind = NodeType::Cons(NodeKind::Assignement);

    if let (Some(rule0), Some(rule1), Some(rule2), Some(rule3)) = (rule0, rule1, rule2, rule3) {
        let children = vec![rule0.into(), rule1.into(), rule2.into(), rule3.into()];
        return Some(Node::new(kind, Some(children)))
    } 
    
    parser.reset(start);
    return None
}
