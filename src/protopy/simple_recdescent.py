from lexer import Token, Lexer
from node import PTNode

class Parser:
    def __init__(self, input: list):
        self.input = input
        self.pointer = 0

    def term(self, tok, node):
        if self.pointer > len(self.input)-1:
            return False
        res = self.input[self.pointer][0] == tok

        if res:
            node.add_child(tok)

        self.pointer += 1
        return res

    def parse(self) -> (bool, PTNode):
        local_node = PTNode("root")
        res = self.add(local_node)
        if res and self.pointer==len(self.input):
            return (True, local_node)
        return (False, local_node)

    def add(self, node):
        save = self.pointer
        return (   self.add_0(save, node)
                or self.add_1(save, node)
                or self.add_2(save, node))

    def add_0(self, save, node):
        self.pointer = save

        local_node = PTNode("add_0")
        res =  (    self.mul(local_node)
                and self.term(Token.ADD, local_node)
                and self.add(local_node))

        if res:
            node.add_child(local_node)

        return res

    def add_1(self, save, node):
        self.pointer = save

        local_node = PTNode("add_1")
        res =  (    self.mul(local_node)
                and self.term(Token.SUB, local_node)
                and self.add(local_node))

        if res:
            node.add_child(local_node)

        return res

    def add_2(self, save, node):
        self.pointer = save

        local_node = PTNode("add_2")
        res =  self.mul(local_node)

        if res:
            node.add_child(local_node)

        return res

    def mul(self, node):
        save = self.pointer
        return (   self.mul_0(save, node)
                or self.mul_1(save, node)
                or self.mul_2(save, node))

    def mul_0(self, save, node):
        self.pointer = save

        local_node = PTNode("mul_0")
        res =  (    self.exp(local_node)
                and self.term(Token.MUL, local_node)
                and self.mul(local_node))

        if res:
            node.add_child(local_node)

        return res

    def mul_1(self, save, node):
        self.pointer = save

        local_node = PTNode("mul_1")
        res =  (    self.exp(local_node)
                and self.term(Token.DIV, local_node)
                and self.mul(local_node))

        if res:
            node.add_child(local_node)

        return res

    def mul_2(self, save, node):
        self.pointer = save

        local_node = PTNode("mul_2")
        res =  self.exp(local_node)

        if res:
            node.add_child(local_node)

        return res

    def exp(self, node):
        save = self.pointer

        return (   self.exp_0(save, node)
                or self.exp_1(save, node))

    def exp_0(self, save, node):
        self.pointer = save

        local_node = PTNode("exp_0")
        res =  (    self.atom(local_node)
                and self.term(Token.EXP, local_node)
                and self.exp(local_node))

        if res:
            node.add_child(local_node)

        return res

    def exp_1(self, save, node):
        self.pointer = save

        local_node = PTNode("exp_1")
        res =  self.atom(local_node)

        if res:
            node.add_child(local_node)

        return res

    def atom(self, node):
        save = self.pointer

        return (self.atom_0(save, node)
                or self.atom_1(save, node))

    def atom_0(self, save, node):
        self.pointer = save

        local_node = PTNode("atom_0")
        res =  (    self.term(Token.LPAREN, local_node)
                and self.add(local_node)
                and self.term(Token.RPAREN, local_node))

        if res:
            node.add_child(local_node)
        return res


    def atom_1(self, save, node):
        self.pointer = save

        local_node = PTNode("atom_1")
        res = self.term(Token.NUMBER, local_node)
        if res:
            node.add_child(local_node)
        return res


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
