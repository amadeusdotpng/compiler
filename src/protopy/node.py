from lexer import Token
from queue import LifoQueue

class PTNode:
    def __init__(self, type, children):
        self.type = type
        self.children = children

    def __str__(self):
        return self.type

def treeString(node):
    res = ""
    Q = LifoQueue()
    Q.put((1, node))
    prevL = 1
    while not Q.empty():
        L, v = Q.get()
        res += ")"*(prevL-L)
        if type(v) == PTNode:
            res += f" ({v}"
            for child in reversed(v.children):
                Q.put((L+1, child))
        elif type(v) == Token:
            res += f" {v.name}"
        prevL = L
    res += ")"
    return res
