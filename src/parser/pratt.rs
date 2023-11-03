use crate::lexer::tokens::{TokenKind, Token};

use super::node::{NodeKind, NodeType, Node};
use super::parser::Parser;

pub fn parse_expression(parser: &mut Parser) -> Node {
    expression(parser, 0)
}

fn expression(parser: &mut Parser, min_bp: u8) -> Node {
    let lhs = parser.lex.next();
    let mut lhs = match is_op(&lhs) {
        /* not an operator */
        false => Node::new(NodeType::Atom(lhs), None),

        /* operator */
        true => match &lhs.kind {
            TokenKind::LPAREN => {
                let lhs = expression(parser, 0);
                assert_eq!(parser.lex.next().kind, TokenKind::RPAREN);
                lhs
            }
            _ => {
                let ((), r_bp) = prefix_bp(&lhs);
                let rhs = expression(parser, r_bp);
                Node::new(NodeType::Atom(lhs), Some(vec![rhs]))
            }
        },
    };

    loop {
        let op = parser.lex.peek();
        if !is_op(&op) {
            if op.kind == TokenKind::EOF { break; }
            panic!("{:?} atom found at {:?}", &op, &op.position)
        }

        if let Some((l_bp, r_bp)) = infix_bp(&op) {
            if l_bp < min_bp {
                break;
            }

            parser.lex.next();
            let rhs = expression(parser, r_bp);
            lhs = Node::new(NodeType::Atom(op), Some(vec![lhs, rhs]));
            continue;
        }
        break;
    }
    lhs
}

fn prefix_bp(tok: &Token) -> ((), u8) {
    match tok.kind {
        TokenKind::BOOL_NOT => ((), 5),
        TokenKind::PLUS
        | TokenKind::BIT_NOT
        | TokenKind::MINUS => ((), 21),

        t => panic!("bad token {:?}", t),
    }
}

fn infix_bp(tok: &Token) -> Option<(u8, u8)> {
    match tok.kind {
        TokenKind::BOOL_OR => Some((1,2)),
        TokenKind::BOOL_AND => Some((3,4)),
        TokenKind::EQ
        | TokenKind::NE
        | TokenKind::LE
        | TokenKind::LT
        | TokenKind::GE
        | TokenKind::GT => Some((7,8)),
        TokenKind::BIT_OR => Some((9,10)),
        TokenKind::BIT_XOR => Some((11,12)),
        TokenKind::BIT_AND => Some((13,14)),

        TokenKind::BIT_LEFT
        | TokenKind::BIT_RIGHT => Some((15,16)),

        TokenKind::PLUS
        | TokenKind::MINUS => Some((17, 18)),

        TokenKind::MULTIPLY
        | TokenKind::MODULUS
        | TokenKind::DIVIDE => Some((19, 20)),

        _ => None
    }
}


fn is_op(tok: &Token) -> bool {
    match tok.kind {
        TokenKind::EOF
        | TokenKind::INT
        | TokenKind::BOOL
        | TokenKind::STR
        | TokenKind::TRUE
        | TokenKind::FALSE
        | TokenKind::ID
        | TokenKind::NUMBER => false,
        _ => true,
    }
}
