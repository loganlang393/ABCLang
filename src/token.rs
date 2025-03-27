//Tokens
//IDENTIFIER(String)
//INTEGER(int)
//LPAREN
//RPAREN
//PLUS
//MINUS
//STAR
//DIV
//EQUALS
//SEMICOLON
//PRINT

trait Token{}

struct Identifier{}
struct LParen{}
struct RParen{}
struct Plus{}
struct Minus{}
struct Star{}
struct Div{}
struct Equals{}
struct Semicolon{}
struct Print{}

impl Token for Identifier{}

impl Token for LParen{}

impl Token for Rparen{}

impl Token for Plus{}

impl Token for Star{}

impl Token for Div{}

impl Token for Equals{}

impl Token for Semicolon{}

impl Token for Print{}
