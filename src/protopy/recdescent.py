# Allows:
# + , -
# * , /

# Does not yet allow
# ^ , % 
# ()
# Negative Numbers

from lexer import Token, Lexer

class PTNode:
    def __init__(self, children):
        self.children = children

class Parser:
    def __init__(self, input: list):
        self.input = input
        self.pointer = 0

    def term(self, tok):
#		print("term_:", "_", self.pointer, tok)
        if self.pointer > len(self.input)-1:
            return False
        res = self.input[self.pointer][0] == tok
        self.pointer += 1
        return res

    def parse(self):
        res = self.add()
        return res and self.pointer==len(self.input)
    
    # ADD

    def add(self):
        save = self.pointer
#		print("add__:", save, self.pointer)
        return (   self.add_0(save)
                or self.add_1(save)
                or self.add_2(save))

    def add_0(self, save):
        self.pointer = save
#		print("add_0:", save, self.pointer)
        return (    self.mul()
                and self.term(Token.ADD)
                and self.add())

    def add_1(self, save):
        self.pointer = save
#		print("add_1:", save, self.pointer)
        return (    self.mul()
                and self.term(Token.SUB)
                and self.add())

    def add_2(self, save):
        self.pointer = save
#		print("add_2:", save, self.pointer)
        return self.mul()

    # MUL

    def mul(self):
        save = self.pointer
#		print("mul__:", save, self.pointer)
        return (   self.mul_0(save)
                or self.mul_1(save)
                or self.mul_2(save)
                or self.mul_3(save)
                or self.mul_4(save)
                or self.mul_5(save))

    def mul_0(self, save):
        self.pointer = save
#		print("mul_0:", save, self.pointer)
        return (    self.int() 
                and self.term(Token.MUL)
                and self.mul())

    def mul_1(self, save):
        self.pointer = save
#		print("mul_1:", save, self.pointer)
        return (    self.int()
                and self.term(Token.DIV)
                and self.mul())

    def mul_2(self, save):
        self.pointer = save
#		print("mul_2:", save, self.pointer)
        return (    self.term(Token.LPAREN)
                and self.add() 
                and self.term(Token.RPAREN)
                and self.term(Token.MUL)
                and self.mul())
        
    def mul_3(self, save):
        self.pointer = save
#		print("mul_3:", save, self.pointer)
        return (    self.term(Token.LPAREN)
                and self.add() 
                and self.term(Token.RPAREN)
                and self.term(Token.DIV)
                and self.mul())

    def mul_4(self, save):
        self.pointer = save
#		print("mul_4:", save, self.pointer)
        return (    self.term(Token.LPAREN)
                and self.add() 
                and self.term(Token.RPAREN))
        
    def mul_5(self, save):
        self.pointer = save
#		print("mul_5:", save, self.pointer)
        return self.int()

    def int(self):
        save = self.pointer
#		print("int__:", save, self.pointer)
        return self.int_0(save)

    def int_0(self, save):
        self.pointer = save
#		print("int_0:", save, self.pointer)
        return self.term(Token.NUMBER)
    
string = "(1+1)*(4*(3+2+(3*9)))"

lex = list(Lexer(string))
parse = Parser(lex)
print("STRING:", string)
print("LEX:\n"+"\n".join([f'({tok}, "{lexeme}")' for tok, lexeme in lex]))
print("RESULT:", parse.parse(), "\n")
