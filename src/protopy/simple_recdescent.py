from lexer import Token, Lexer
from node import PTNode

class Parser:
    def __init__(self, input: list):
        self.input = input
        self.pointer = 0

    def term(self, tok):
        if self.pointer > len(self.input)-1:
            return None
        res = self.input[self.pointer][0] == tok
        self.pointer += 1
        if res:
            return tok
        return None

    def mark(self):
        return self.pointer

    def reset(self, pos):
        self.pointer = pos

    def parse(self) -> (bool, PTNode):
        if (tree := self.add()) and self.pointer==len(self.input):
            return (True, tree)
        return (False, tree)

    def add(self):
        save = self.mark()

        self.reset(save)
        if (prod:=self.add_0()):
            return PTNode("add_0", prod)
        self.reset(save)
        if (prod:=self.add_1()):
            return PTNode("add_1", prod)
        self.reset(save)
        if (prod:=self.add_2()):
            return PTNode("add_2", prod)
        return None

    def add_0(self):
        res = ((mul0:= self.mul())
            and(tok0:= self.term(Token.ADD))
            and(add0:= self.add()))
        if res:
            return [mul0, tok0, add0]
        return None

    def add_1(self):
        res = ((mul0:= self.mul())
            and(tok0:= self.term(Token.SUB))
            and(add0:= self.add()))
        if res:
            return [mul0, tok0, add0]
        return None

    def add_2(self):
        res = (mul0:= self.mul())
        if res:
            return [mul0]
        return None

    def mul(self):
        save = self.mark()

        self.reset(save)
        if (prod:=self.mul_0()):
            return PTNode("mul_0", prod)
        self.reset(save)
        if (prod:=self.mul_1()):
            return PTNode("mul_1", prod)
        self.reset(save)
        if (prod:=self.mul_2()):
            return PTNode("mul_2", prod)
        return None

    def mul_0(self):
        res = ( (exp0:= self.exp())
            and (tok0:= self.term(Token.MUL))
            and (mul0:= self.mul()))
        if res:
            return [exp0, tok0, mul0]
        return None

    def mul_1(self):
        res = ( (exp0:= self.exp())
            and (tok0:= self.term(Token.DIV))
            and (mul0:= self.mul()))
        if res:
            return [exp0, tok0, mul0]
        return None

    def mul_2(self):
        res = (exp0:= self.exp())
        if res:
            return [exp0]
        return None

    def exp(self):
        save = self.mark()

        self.reset(save)
        if (prod:=self.exp_0()):
            return PTNode("exp_0", prod)
        self.reset(save)
        if (prod:=self.exp_1()):
            return PTNode("exp_1", prod)
        return None

    def exp_0(self):
        res = ( (atom0:= self.atom())
            and (tok0 := self.term(Token.EXP))
            and (exp0 := self.exp()))
        if res:
            return [atom0, tok0, exp0]
        return None

    def exp_1(self):
        res = (atom0 := self.atom())
        if res:
            return [atom0]
        return None

    def atom(self):
        save = self.mark()

        self.reset(save)
        if (prod:=self.atom_0()):
            return PTNode("atom_0", prod)
        self.reset(save)
        if (prod:=self.atom_1()):
            return PTNode("atom_1", prod)
        return None

    def atom_0(self):
        res = ( (tok0:= self.term(Token.LPAREN))
            and (add0:= self.add())
            and (tok1:= self.term(Token.RPAREN)))
        if res:
            return [tok0, add0, tok1]
        return None

    def atom_1(self):
        res = (tok0 := self.term(Token.NUMBER))
        if res:
            return [tok0]
        return None

if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('filepath')
    args = parser.parse_args()

    inp = open(args.filepath).read()
    lex = list(Lexer(inp))

    res, tree = Parser(lex).parse()
    
    print(f"STRING: {inp}")
    print("LEX:\n"+"\n".join([f'({tok}, "{lexeme}")' for tok, lexeme in lex])+"\n")
    print(f"RESULT: {res}")
    print(f"RESULT:\n{tree}\n")
