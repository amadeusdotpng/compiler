import re
s = "while;this_is_an_identiferkdnfvkj;;1212;;"
cs = ""
c_tok = "TOKEN::ERROR"
tokens = []

def match_string(substring):
    if re.match(r"^while$", substring):
        return "TOKEN::WHILE"
    elif re.match(r"^;$", substring):
        return "TOKEN::SEMICOLON"
    elif re.match(r"^[a-zA-Z_]+$", substring):
        return "TOKEN::IDENTIFIER"
    else:
        return "TOKEN::ERROR"

while s:
    cs, s = cs+s[0], s[1:] # drain
    t_tok = match_string(cs) # get token

    if t_tok == "TOKEN::ERROR":

        if c_tok != "TOKEN::ERROR":
            s = cs[-1]+s
            tokens.append((cs[:-1], c_tok))
        else:
            tokens.append((cs, c_tok))
        c_tok = "TOKEN::ERROR"
        cs = ""
    else:
        c_tok = t_tok

if c_tok != "TOKEN::ERROR":
    tokens.append((cs, c_tok))

print(tokens)
