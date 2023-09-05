import re

class Lexer:
    def __init__(self, input: str):
        self.input = input
        self.curr_pos = 0
        self.read_pos = 1

    def next_token(self) -> (str, str):
        tok = "TOKEN::ERROR"

        if self.read_pos > len(self.input):
            return ("", "TOKEN::EOF")

        while True:
            sub = self.input[self.curr_pos:self.read_pos]
            t_tok = self.match_string(sub)

            if self.read_pos > len(self.input):
                return (sub, tok) 
            
            elif t_tok == "TOKEN::ERROR":
                if tok == "TOKEN::ERROR":
                    self.read_pos += 1
                    self.curr_pos = self.read_pos-1
                    return (sub, tok)
                else:
                    self.curr_pos = self.read_pos-1
                    return (sub[:-1], tok)
            else:
                tok = t_tok
                self.read_pos += 1
                
    
    def match_string(self, sub: str) -> str:
        if re.match(r"^while$", sub):
            return "TOKEN::WHILE"
        elif re.match(r"^;$", sub):
            return "TOKEN::SEMICOLON"
        elif re.match(r"^[a-zA-Z_]+$", sub):
            return "TOKEN::IDENTIFIER"
        else:
            return "TOKEN::ERROR"


inp = "while;identifer;32202;"
lex = Lexer(inp)

while True:
    lexeme, token = lex.next_token()
    print((lexeme, token))
    if token == "TOKEN::EOF":
        break
