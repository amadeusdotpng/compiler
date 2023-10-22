from lexer import Token, Lexer
from node import PTNode, treeString

class Parser:
    def __init__(self, lex: Lexer):
        self.lex = lex

    def mark(self):
        return self.lex.mark()

    def reset(self, pos):
        self.lex.reset(pos)

    def expect(self, tok):
        peek = self.lex.peek_token()[0]
        if peek == tok:
            return self.lex.next_token()[0]
        return None

    def parse(self):
        res = bool((add0 := self.add()) and
                   (tok0 := self.expect(Token.EOF)))
        if add0:
            return (res, PTNode("prog", [add0, tok0]))
        return (res, None)

    def add(self):
        left = self.mul()
        while ((tok0:=self.expect(Token.ADD)) or
               (tok0:=self.expect(Token.SUB))):
            right = self.mul()
            left = PTNode("add", [left, tok0, right])

        return left

    def mul(self):
        left = self.exp()
        while ((tok0:=self.expect(Token.MUL)) or
               (tok0:=self.expect(Token.DIV))):
            right = self.exp()
            left = PTNode("mul", [left, tok0, right])

        return left

    def exp(self):
        right = self.atom()
        while (tok0:=self.expect(Token.EXP)):
            left = self.atom()
            right = PTNode("exp", [left, tok0, right])

        return right

    def atom(self):
        save = self.mark()

        self.reset(save)
        if (prod:=self.atom_0()):
            return PTNode("atom_0", prod)

        self.reset(save)
        if (prod:=self.atom_1()):
            return PTNode("atom_1", prod)

        self.reset(save)
        return None

    def atom_0(self):
        res = bool((tok0 := self.expect(Token.LPAREN)) and
                   (add0 := self.add()) and
                   (tok1 := self.expect(Token.RPAREN)))
        if res:
            return [tok0, add0, tok1]
        return None

    def atom_1(self):
        res = bool((tok0 := self.expect(Token.NUMBER)))
        if res:
            return [tok0]
        return None

if __name__ == '__main__':
    inp = open("test.txt").read()
    lex = Lexer(inp)
    res, tree = Parser(lex).parse()

    print(f"STRING: {inp}")
    print(f"RESULT: {res}")
    print(f"TREE:\n{treeString(tree)}\n")
