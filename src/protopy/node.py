from lexer import Token

class PTNode:
    def __init__(self, type, children):
        self.type = type
        self.children = children

    def __str__(self):
        return self.tree_string(0)

    def tree_string(self, level):
        indent = level*"  "
        if type(self.type) == str:
            res = f"{indent}({self.type} "
        else:
            res = f"{indent}({self.type.name} "
        for child in self.children:
            if type(child) == Token:
                res += f"\n{indent+'  '}{child.name}"
            else:
                res += f"\n{child.tree_string(level+1)}"
        res += f"\n{indent})"
        return res

