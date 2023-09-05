import re
s = "while;identifier;1234;"
# print(f"str length: {len(s)}")

curr_pos = 0
read_pos = 1

tokens = []
tok = "TOKEN::ERROR"

def match_string(substring):
    if re.match(r"^while$", substring):
        return "TOKEN::WHILE"
    elif re.match(r"^;$", substring):
        return "TOKEN::SEMICOLON"
    elif re.match(r"^[a-zA-Z_]+$", substring):
        return "TOKEN::IDENTIFIER"
    else:
        return "TOKEN::ERROR"

while True:
    t_tok = match_string(s[curr_pos:read_pos])
#    print((t_tok, s[curr_pos:read_pos], curr_pos, read_pos))
    if read_pos > len(s):
        if s[curr_pos:read_pos-1]: 
            tokens.append((s[curr_pos:read_pos-1], tok))
        break

    elif t_tok == "TOKEN::ERROR":
        if tok == "TOKEN::ERROR":
            tokens.append((s[curr_pos:read_pos], tok))
            read_pos += 1
        else:
            tokens.append((s[curr_pos:read_pos-1], tok))

        curr_pos = read_pos-1;
        tok = "TOKEN::ERROR"
    else:
        tok = t_tok
        read_pos += 1


print()
for pair in tokens:
    print(pair)
