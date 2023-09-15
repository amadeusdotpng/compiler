# Cheetah
## Ultimate Goal
In the end, the language should be a basic, statically types, expression-based, script-like,
compiled language as powerful as basic Python.

## Project Structure
- `/papers` contains helpful reading resources to learn more about compilers.
- `/src/protopy` contains Python prototypes of the components of the compiler.
- `/src/lexer/` contains  the Rust code for the lexer of the compiler.

## Current State
This project is still a Work In Progress. There is currently only a lexer.
There are Python prototypes for the lexer and for the future recursive descent parser.

## Todo
- Parsing
- Semantic Analysis
- Code Generation

## Helpful Reading Resources

### General
- [Tristan Hume - Writing a Compiler in Rust](https://thume.ca/2019/04/18/writing-a-compiler-in-rust/)

### Lexing
- [Indentation Sensitive Parsing for Parsec](./papers/Indentation-Sensitive_Parsing_for_Parsec.pdf)

### Parsing
- [Laurent Tratt - Which Parsing Approach](https://tratt.net/laurie/blog/2020/which_parsing_approach.html)
- [Bryan Ford - Packrat Parsing](https://pdos.csail.mit.edu/~baford/packrat/thesis/)
- [PEP 617](https://peps.python.org/pep-0617/)
- [Indentation Sensitive Parsing for Parsec](./papers/Indentation-Sensitive_Parsing_for_Parsec.pdf)
- [Packrat Parsers can Support Left Recursion](./papers/Packrat_Parsers_Can_Support_Left_Recursion.pdf)
- [Left Recursion in Parsing Expression Grammars](./papers/Left_Recursion_in_Parsing_Expression_Grammars.pdf)
