use crate::lexer::{tokens::{Token, TokenKind}, lex::Lexer};
use super::node::{NodeKind, Node};
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

    pub fn memoize(&mut self, f: fn(&mut PackratParser) -> Option<Node>, kind: NodeKind) -> Option<Node> {
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
        NodeKind::Prog.parse(self)
    }
}


