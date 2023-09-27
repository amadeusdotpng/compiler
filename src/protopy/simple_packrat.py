from lexer import Token, Lexer
from node import PTNode


class Parser:
    def __init__(self, input: list):
        self.input = input
        self.pointer = 0
        self.memo_table = {}

    def mark(self):
        print(f"marking:{self.pointer}")
        return self.pointer

    def reset(self, pos):
        print(f"resetting:{pos}")
        self.pointer = pos

#    def get_memo(self, func):
#        pass

    # use this function in replacement of left-recursive non-terminals in productions
    def memoize(self, func):
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

                # This is where the pointer will be after going through func()
                endpos = self.mark() 

                # If the parse length is shorter, we break and 
                # don't store the result in the memo_table
                print(f"loop:\nres:\n{res}\nendpos:{endpos}\nlastres:{lastres}\nlastpos:{lastpos}")
                if endpos <= lastpos or endpos >= len(self.input):
                    break

                print("caching res\n")
                self.memo_table[key] = lastres, lastpos = res, endpos 

            # Make sure to return the proper result

            print("exit loop")
            res = lastres
            self.reset(lastpos)

        print(f"res:\n{res}")
        print(f"pointer: {self.pointer}")
        print("returning memoize\n")
        return res

    def parse(self) -> (bool, PTNode):
        if (tree := self.add()) and self.pointer==len(self.input):
            return (True, tree)
        return (False, tree)

    def term(self, tok):
        print("enter term")
        if self.pointer > len(self.input)-1:
            print("exiting term")
            return None
        res = self.input[self.pointer][0] == tok
        print(f"res:{res}, pointer:{self.pointer}, tok:{tok}, inptok:{self.input[self.pointer][0]}")
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
            return PTNode("add_0", prod)
        self.reset(save)
        if (prod:=self.add_1()):
            return PTNode("add_1", prod)
        self.reset(save)
        if (prod:=self.add_2()):
            return PTNode("add_2", prod)
        return None

    def add_0(self):
        print("enter add_0")
        res = bool((add0 := self.memoize(self.add))
            and(tok0 := self.term(Token.ADD))
            and(atom0:= self.atom()))
        print("in add_0")
        print(f"res:{res}")
        print("exiting add_0\n")
        if res:
            return [add0, tok0, atom0]
        return None

    def add_1(self):
        print("enter add_1")
        res = bool((add0 := self.memoize(self.add))
            and(tok0 := self.term(Token.SUB))
            and(atom0:= self.atom()))
        print("in add_1")
        print(f"res:{res}")
        print("exiting add_1\n")
        if res:
            return [add0, tok0, atom0]
        return None

    def add_2(self):
        print("enter add_2")
        res = bool(atom0:= self.atom())
        print("in add_2")
        print(f"res:{res}")
        print("exiting add_2\n")
        if res:
            return [atom0]
        return None

    def atom(self):
        print("enter atom")
        save = self.mark()

        self.reset(save)
        if (prod:=self.atom_0()):
            return PTNode("atom_0", prod)
        self.reset(save)
        if (prod:=self.atom_1()):
            return PTNode("atom_1", prod)
        return None

    def atom_0(self):
        print("enter atom_0")
        res = bool( (tok0:= self.term(Token.LPAREN))
            and (add0:= self.memoize(self.add))
            and (tok1:= self.term(Token.RPAREN)))
        print("in atom_0")
        print(f"res:{res}")
        print("exiting atom_0\n")
        if res:
            return [tok0, add0, tok1]
        return None

    def atom_1(self):
        print("enter atom_1")
        res = bool(tok0 := self.term(Token.NUMBER))
        print("in atom_1")
        print(f"res:{res}")
        print("exiting atom_1\n")
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
    print("LEX:\n"+"\n".join([f'({tok}, {repr(lexeme)})' for tok, lexeme in lex])+"\n")
    print(f"RESULT: {res}")
    print(f"RESULT:\n{tree}\n")
