import re
from enum import Enum

class Token(Enum):
    ADD = "+"
    SUB = "-"
    MOD = "%"
    MUL = "*"
    DIV = "/"
    EXP = "^"
    LPAREN = "("
    RPAREN = ")"
    NUMBER = "NUMBER"
    EOF = "EOF"
    ERROR = "ERROR"
    WHITESPACE = "WHITESPACE"
    
class Lexer:
    def __init__(self, input: str):
        self.input = input
        self.curr_pos = 0
        self.read_pos = 1

    def mark(self):
        return (self.curr_pos, self.read_pos);

    def reset(self, pos):
        self.curr_pos, self.read_pos = pos;

    def next_token(self):
        tok = Token.ERROR

        if self.read_pos > len(self.input):
            return (Token.EOF, "")

        while True:
            sub = self.input[self.curr_pos:self.read_pos]
            t_tok = self.match_string(sub)
            
            if self.read_pos > len(self.input):
                if tok == Token.WHITESPACE:
                    return (Token.EOF, "")
                return (tok, sub) 
            
            elif t_tok == Token.ERROR:
                if tok == Token.ERROR:
                    self.read_pos += 1
                    self.curr_pos = self.read_pos-1
                    return (tok, sub)
                elif tok == Token.WHITESPACE:   # skip whitespace
                    self.curr_pos = self.read_pos-1
                    continue
                else:
                    self.curr_pos = self.read_pos-1
                    return (tok, sub[:-1])
            else:
                tok = t_tok
                self.read_pos += 1

    def peek_token(self):
        last_pos = (self.curr_pos, self.read_pos)
        res = self.next_token()
        self.curr_pos, self.read_pos = last_pos
        return res
    
    def match_string(self, sub: str) -> str:
        if re.match(r"^\+\Z", sub):
            return Token.ADD
        elif re.match(r"^\-\Z", sub):
            return Token.SUB
        elif re.match(r"^%\Z", sub):
            return Token.MOD
        elif re.match(r"^\*\Z", sub):
            return Token.MUL
        elif re.match(r"^/\Z", sub):
            return Token.DIV
        elif re.match(r"^\^\Z", sub):
            return Token.EXP
        elif re.match(r"^\(\Z", sub):
            return Token.LPAREN
        elif re.match(r"^\)\Z", sub):
            return Token.RPAREN
        elif re.match(r"^[0-9]+\Z", sub):
            return Token.NUMBER
        elif re.match(r"^\s+\Z", sub):
            return Token.WHITESPACE
        else:
            return Token.ERROR

    def __next__(self):
        token, lexeme = self.next_token()
        if token == Token.EOF:
            raise StopIteration
        return (token, lexeme)


    def __iter__(self):
        return self
