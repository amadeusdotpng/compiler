from lexer import Token, Lexer
from node import PTNode, treeString

class Parser:
    def __init__(self, lex: Lexer):
        self.lex = lex
        self.memo_table = {}

    def mark(self):
        return self.lex.mark()

    def reset(self, pos):
        self.lex.reset(pos)

    def expect(self, tok):
        peek = self.lex.peek_token()[0]
        if peek == tok:
            return self.lex.next_token()[0]
        return None

    def left_memoize(self, func):
        save = self.mark()

        key = (func, save)
        if key in self.memo_table:
            res, endpos = self.memo_table[key];
            self.reset(endpos)
        else:
            self.memo_table[key] = lastres, lastpos = None, save
            while True:
                self.reset(save)

                res = func()
                endpos = self.mark()

                if endpos[1] <= lastpos[1]:
                    break

                self.memo_table[key] = lastres, lastpos = res, endpos

            res = lastres
            self.reset(lastpos)
        return res

    def parse(self):
        res = bool((add0 := self.left_memoize(self.add)) and
                   (tok0 := self.expect(Token.EOF)))

        if add0:
            return (res, PTNode("prog", [add0, tok0]))
        return (res, None)

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

        self.reset(save)
        return None

    def add_0(self):
        res = bool((add0 := self.left_memoize(self.add)) and
                   (tok0 := self.expect(Token.ADD)) and
                   (mul0 := self.left_memoize(self.mul)))
        if res:
            return [add0, tok0, mul0]
        return None

    def add_1(self):
        res = bool((add0 := self.left_memoize(self.add)) and
                   (tok0 := self.expect(Token.SUB)) and
                   (mul0 := self.left_memoize(self.mul)))
        if res:
            return [add0, tok0, mul0]
        return None

    def add_2(self):
        res = bool((mul0 := self.left_memoize(self.mul)))
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

        self.reset(save)
        return None

    def mul_0(self):
        res = bool((mul0 := self.left_memoize(self.mul)) and
                   (tok0 := self.expect(Token.MUL)) and
                   (exp0 := self.exp()))
        if res:
            return [mul0, tok0, exp0]
        return None

    def mul_1(self):
        res = bool((mul0 := self.left_memoize(self.mul)) and
                   (tok0 := self.expect(Token.DIV)) and
                   (exp0 := self.exp()))
        if res:
            return [mul0, tok0, exp0]
        return None

    def mul_2(self):
        res = bool((exp0 := self.exp()))
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

        self.reset(save)
        return None

    def exp_0(self):
        res = bool((atom0:= self.atom()) and
                   (tok0 := self.expect(Token.EXP)) and
                   (exp0 := self.exp()))
        if res:
            return [atom0, tok0, exp0]
        return None

    def exp_1(self):
        res = bool((atom0:= self.atom()))
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

        self.reset(save)
        return None

    def atom_0(self):
        res = bool((tok0 := self.expect(Token.LPAREN)) and
                   (add0 := self.left_memoize(self.add)) and
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
