# Allows:
# + , -
# * , /

# Does not yet allow
# ^ , % 
# ()
# Negative Numbers

from lexer import Token, Lexer

class PTNode:
    def __init__(self, name):
        self.name = name
        self.children = []

    def add_child(self, tok):
        self.children.append(tok)

    def __str__(self):
        return self.tree_string(0)

    def tree_string(self, level):
        indent = level*"  "
        res = f"{indent}{self.name}:["
        for child in self.children:
            if type(child) == Token:
                res += f"\n{indent+'  '}{child}"
            else:
                res += f"\n{child.tree_string(level+1)}"
        res += f"\n{indent}]"
        return res



class Parser:
    def __init__(self, input: list):
        self.input = input
        self.pointer = 0

    def term(self, tok, node):
#		print("term_:", "_", self.pointer, tok)
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
    
    # ADD

    def add(self, node):
        save = self.pointer
#        print("add__:", save, self.pointer)
        return (   self.add_0(save, node)
                or self.add_1(save, node)
                or self.add_2(save, node))

    def add_0(self, save, node):
        self.pointer = save
#        print("\nadd_0:", save, self.pointer)
        
        local_node = PTNode("add_0")
        res =  (    self.mul(local_node)
                and self.term(Token.ADD, local_node)
                and self.add(local_node))
        if res:
            node.add_child(local_node)
        
        return res

    def add_1(self, save, node):
        self.pointer = save
#        print("\nadd_1:", save, self.pointer)

        local_node = PTNode("add_1")
        res =  (    self.mul(local_node)
                and self.term(Token.SUB, local_node)
                and self.add(local_node))

        if res:
            node.add_child(local_node)

        return res

    def add_2(self, save, node):
        self.pointer = save
#        print("\nadd_2:", save, self.pointer)

        local_node = PTNode("add_2")
        res = self.mul(local_node)

        if res:
            node.add_child(local_node)

        return res

    # MUL

    def mul(self, node):
        save = self.pointer
#        print("mul__:", save, self.pointer)
        return (   self.mul_0(save, node)
                or self.mul_1(save, node)
                or self.mul_2(save, node)
                or self.mul_3(save, node)
                or self.mul_4(save, node)
                or self.mul_5(save, node))

    def mul_0(self, save, node):
        self.pointer = save
#        print("mul_0:", save, self.pointer)

        local_node = PTNode("mul_0")
        res =  (    self.int(local_node) 
                and self.term(Token.MUL, local_node)
                and self.mul(local_node))

        if res:
            node.add_child(local_node)

        return res

    def mul_1(self, save, node):
        self.pointer = save
#        print("mul_1:", save, self.pointer)

        local_node = PTNode("mul_1")
        return (    self.int(local_node)
                and self.term(Token.DIV, local_node)
                and self.mul(local_node))

    def mul_2(self, save, node):
        self.pointer = save
#        print("mul_2:", save, self.pointer)

        local_node = PTNode("mul_2")
        res =  (    self.term(Token.LPAREN, local_node)
                and self.add(local_node) 
                and self.term(Token.RPAREN, local_node)
                and self.term(Token.MUL, local_node)
                and self.mul(local_node))

        if res:
            node.add_child(local_node)

        return res
        
    def mul_3(self, save, node):
        self.pointer = save
#        print("mul_3:", save, self.pointer)

        local_node = PTNode("mul_3")
        res =  (    self.term(Token.LPAREN, local_node)
                and self.add(local_node) 
                and self.term(Token.RPAREN, local_node)
                and self.term(Token.DIV, local_node)
                and self.mul(local_node))

        if res:
            node.add_child(local_node)

        return res

    def mul_4(self, save, node):
        self.pointer = save
#        print("mul_4:", save, self.pointer)

        local_node = PTNode("mul_4")
        res =  (    self.term(Token.LPAREN, local_node)
                and self.add(local_node) 
                and self.term(Token.RPAREN, local_node))

        if res:
            node.add_child(local_node)

        return res
        
    def mul_5(self, save, node):
        self.pointer = save
#        print("mul_5:", save, self.pointer)

        local_node = PTNode("mul_5")
        res =  self.int(local_node)

        if res:
            node.add_child(local_node)

        return res

    # INT

    def int(self, node):
        save = self.pointer
#        print("int__:", save, self.pointer)
        return self.int_0(save, node)

    def int_0(self, save, node):
        self.pointer = save
#        print("int_0:", save, self.pointer)

        local_node = PTNode("int_0")
        res =  self.term(Token.NUMBER, local_node)

        if res:
            node.add_child(local_node)

        return res
    

if __name__ == '__main__':
#    import time
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('filepath')
    args = parser.parse_args()
    
    inp = open(args.filepath).read()
    lex = list(Lexer(inp))

#    s = time.time_ns()
    res, tree = Parser(lex).parse()
#    t = (time.time_ns()-s)/1000000

    print(f"STRING: {inp}")
    print("LEX:\n"+"\n".join([f'({tok}, "{lexeme}")' for tok, lexeme in lex])+"\n")
    print(f"RESULT: {res}")
    print(f"RESULT:\n{tree}\n")
#    print(f"{t}ms")
