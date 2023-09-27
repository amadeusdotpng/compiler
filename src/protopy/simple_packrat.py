from lexer import Token, Lexer
from node import PTNode


class Parser:
    def __init__(self, input: list):
        self.input = input
        self.pointer = 0
        self.memo_table = {}

    def mark(self):
        print(f"mark: {self.pointer}")
        return self.pointer

    def reset(self, pos):
        print(f"reset: {pos}")
        self.pointer = pos

    # use this function in replacement of left-recursive non-terminals in productions
    def left_rec_memoize(self, func):
        print("enter memoize")
        save = self.mark()
        key = (save, func)

        if key in self.memo_table:
            print("key in memo_table")
            res, endpos = self.memo_table[key]
            self.reset(endpos) # Skip Ahead
        else:
            print("key not in memo_table")
            # Fails at first run
            self.memo_table[key] = lastres, lastpos = None, save 
            while True:
                print("enter loop")
                self.reset(save)
                res = func()
                print("in loop")

                # This is where the pointer will be after going through func()
                endpos = self.mark() 

                # If the parse length is shorter, we break and 
                # don't store the result in the memo_table
                print(f"loop:\nres:\n{res}\nendpos:{endpos}\nlastres:{lastres}\nlastpos:{lastpos}")
                if endpos <= lastpos or endpos >= len(self.input):
                    print("exiting loop")
                    break

                print("caching")
                self.memo_table[key] = lastres, lastpos = res, endpos 

            # Make sure to return the proper result
            res = lastres
            self.reset(lastpos)
            print(f"returning:\nres:\n{res}\npointer:{self.pointer}")
        return res

    def parse(self) -> (bool, PTNode):
        if (tree := self.add()) and self.pointer==len(self.input):
            return (True, tree)
        return (False, tree)

    def term(self, tok):
        print("enter term")
        if self.pointer > len(self.input)-1:
            return None
        res = self.input[self.pointer][0] == tok
        print(f"{self.input[self.pointer][0]}=={tok}:{res}")
        self.pointer += 1
        print("exiting term")
        if res:
            return tok
        return None

    def add(self):
        print("enter add")
        save = self.mark()

        self.reset(save)
        if (prod:=self.add_0()):
            print("in add: returning add_0")
            return PTNode("add_0", prod)
        self.reset(save)
        if (prod:=self.add_1()):
            print("in add: returning add_1")
            return PTNode("add_1", prod)
        self.reset(save)
        if (prod:=self.add_2()):
            print("in add: returning add_2")
            return PTNode("add_2", prod)
        self.reset(save)
        print("in add: returning None")
        return None

    def add_0(self):
        print("enter add_0")
        res = bool(   (add0 := self.left_rec_memoize(self.add))
                   and(tok0 := self.term(Token.ADD))
                   and(mul0 := self.mul()))
        print(f"in add_0: {res}\nexiting add_0")
        if res:
            return [add0, tok0, mul0]
        return None

    def add_1(self):
        print("enter add_1")
        res = bool(   (add0 := self.left_rec_memoize(self.add))
                   and(tok0 := self.term(Token.SUB))
                   and(mul0 := self.mul()))
        print(f"in add_1: {res}\nexiting add_1")
        if res:
            return [add0, tok0, mul0]
        return None

    def add_2(self):
        print("enter add_2")
        res = bool(mul0:= self.mul())
        print(f"in add_2: {res}\nexiting add_2")
        if res:
            return [mul0]
        return None

    def mul(self):
        print("enter mul")
        save = self.mark()

        self.reset(save)
        if (prod:=self.mul_0()):
            print("in mul: returning mul_0")
            return PTNode("mul_0", prod)
        self.reset(save)
        if (prod:=self.mul_1()):
            print("in mul: returning mul_0")
            return PTNode("mul_1", prod)
        self.reset(save)
        if (prod:=self.mul_2()):
            return PTNode("mul_2", prod)
        self.reset(save)
        return None

    def mul_0(self):
        print("enter mul_0")
        res = bool(   (mul0 := self.left_rec_memoize(self.mul))
                   and(tok0 := self.term(Token.MUL))
                   and(atom0:= self.atom()))
        print(f"in mul0: {res}\nexiting mul_0")
        if res:
            return [mul0, tok0, atom0]
        return None

    def mul_1(self):
        print("enter mul_1")
        res = bool(   (mul0 := self.left_rec_memoize(self.mul))
                   and(tok0 := self.term(Token.DIV))
                   and(atom0:= self.atom()))
        print(f"in mul1: {res}\nexiting mul_1")
        if res:
            return [mul0, tok0, atom0]
        return None

    def mul_2(self):
        print("enter mul_2")
        res = bool(atom0:= self.atom())
        print(f"in mul2: {res}\nexiting mul_2")
        if res:
            return [atom0]
        return None

    def atom(self):
        print("enter atom")
        save = self.mark()

        self.reset(save)
        if (prod:=self.atom_0()):
            print("in atom: returning atom_0")
            return PTNode("atom_0", prod)
        self.reset(save)
        if (prod:=self.atom_1()):
            print("in atom: returning atom_1")
            return PTNode("atom_1", prod)
        self.reset(save)
        print("in atom: returning None")
        return None

    def atom_0(self):
        print("enter atom_0")
        res = bool( (tok0:= self.term(Token.LPAREN))
            and (add0:= self.left_rec_memoize(self.add))
            and (tok1:= self.term(Token.RPAREN)))
        print(f"in atom_0: {res}\nexiting atom_0")
        if res:
            return [tok0, add0, tok1]
        return None

    def atom_1(self):
        print("enter atom_1")
        res = bool(tok0 := self.term(Token.NUMBER))
        print(f"in atom_1: {res}\nexiting atom_1")
        if res:
            return [tok0]
        return None

if __name__ == '__main__':
    inp = open("test.txt").read()
    lex = list(Lexer(inp))

    res, tree = Parser(lex).parse()
    
    print(f"STRING: {inp}")
    print("LEX:\n"+"\n".join([f'({tok}, {repr(lexeme)})' for tok, lexeme in lex])+"\n")
    print(f"RESULT: {res}")
    print(f"RESULT:\n{tree}\n")
