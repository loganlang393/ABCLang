#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum Token {
    // holdings
    Identifier(String),
    funcName(String),
    structName(String),
    Integer(i32),
    Bool(bool),
    Null,
    // Keywords
    kwInt,
    kwBool,
    kwVoid,
    kwStruct,
    kwFunc,
    kwBreak,
    kwReturn,
    kwIf,
    kwWhile,
    kwPrint,
    kwVarDec,
   // kwStmnt,
    kwBlock,
    kwCall,
    kwNew,
    
    // opps
    lParen,
    rParen,
    Plus,
    Minus,
    Star,
    Div,
    Equal,
    Semicolon,
    And,
    Or,
    Eof,
}

pub trait TokenTrait {
    fn toString(&self) -> String;
}

impl Token {
    pub fn toString(&self) -> String {
        match self {
            Token::Identifier( s) => s.clone(),
            Token::funcName( s) => s.clone(),
            Token::structName( s) => s.clone(),
            Token::Integer(i) => i.to_string(),
            Token::Bool(b) => b.to_string(),
            Token::Null => "null".to_string(),
            Token::kwInt => "int".to_string(),
            Token::kwBool => "bool".to_string(),     
            Token::kwVoid => "void".to_string(),
            Token::kwStruct => "struct".to_string(),
            Token::kwFunc => "func".to_string(),
            Token::kwBreak => "break".to_string(),
            Token::kwReturn => "return".to_string(),
            Token::kwIf => "if".to_string(),
            Token::kwWhile => "while".to_string(),
            Token::kwPrint => "println".to_string(),
            Token::kwVarDec => "vardec".to_string(),
            //Token::kwStmnt => "stmnt".to_string(),
            Token::kwBlock => "block".to_string(),
            Token::kwCall => "call".to_string(),
            Token::kwNew => "new".to_string(),
            Token::lParen => "(".to_string(),
            Token::rParen => ")".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Equal => "=".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::And => "and".to_string(),
            Token::Or => "or".to_string(),
            Token::Eof => "EOF".to_string(),
        }
    }
}
