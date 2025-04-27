use crate::token::Token;
use std::process;


pub struct Tokenizer{
    tokes: Vec<char>, //Vec<char> cause I need indexing
    pos: usize,
    eof_flag: bool,
    tab: i32,
}

impl Tokenizer {
    pub fn newToken(tokes: &str) -> Self{
        Tokenizer {
            tokes: tokes.chars().collect(),// converts that string into vec<char>
            pos: 0, //position starts on 0
            eof_flag: false,
            tab: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token>{
        let mut tokens = Vec::new();
        
        while let Some(curr) = self.readToken(){
            tokens.push(curr);
        }

        return tokens;
    }
    
    // Returns index of token
    fn currPosition (&self) -> Option<char> {
        self.tokes.get(self.pos).copied()
    }

    //Skips whitespace
    fn skips (&mut self) {
        while let Some(curr) = self.currPosition(){
            if curr == '\t'{
                self.tab += 1;
            }else if !curr.is_whitespace(){
                break;
            } 
            self.forwardTokes();
        }
        
    }

    //forwarder
    pub fn forwardTokes(&mut self) {
        if self.pos < self.tokes.len(){
            self.pos += 1;
        }
        
    }

    fn readInteger(&mut self) -> Token {
        let mut digit = String::new();

        while let Some(curr) = self.currPosition() {
            if curr.is_digit(10){
                digit.push(curr);
                self.forwardTokes();
            } else{
                break;
            }
        }
        Token::Integer(digit.parse().unwrap())
    }

    fn identifiers(&mut self) -> Token{
        let mut identifier = String::new();
        while let Some(curr) = self.currPosition(){
            if curr.is_alphanumeric() { //identifiers only characters and number)
                identifier.push(curr);
                self.forwardTokes();
            } else{
                break;
            }
        }
        Token::Identifier(identifier)
    }
    //Still work in progress
    pub fn readToken(&mut self) -> Option<Token>{
        self.skips();
        if self.eof_flag && self.pos >= self.tokes.len(){
            return None;
        }
        match self.currPosition() {
            Some(curr) => match curr {
                '(' => {
                    self.forwardTokes();
                    Some(Token::lParen)
                }
                ')' => {
                    self.forwardTokes();
                    Some(Token::rParen)
                }
                '+' => {
                    self.forwardTokes();
                    Some(Token::Plus)
                }
                '-' => {
                    self.forwardTokes();
                    Some(Token::Minus)
                }
                '*' => {
                    self.forwardTokes();
                    Some(Token::Star)
                }
                '/' => {
                    self.forwardTokes();
                    Some(Token::Div)
                }
                '=' => {
                    self.forwardTokes();
                    if(self.currPosition()? == '='){
                        self.forwardTokes();
                        Some(Token::Equals)
                    }else{
                        let currTab = self.tab.clone();
                        self.tab = 0;
                        Some(Token::kwEqual(currTab))
                    }
                }
                '!' => {
                    self.forwardTokes();
                    if(self.currPosition()? == '='){
                        self.forwardTokes();
                        Some(Token::NotEquals)
                    }else{
                        Some(Token::Not)
                    }
                }
                ';' => {
                    self.forwardTokes();
                    Some(Token::Semicolon)
                }
                '<' => {
                    self.forwardTokes();
                    if(self.currPosition()? == '='){
                        self.forwardTokes();
                        Some(Token::LessEqual)
                    }else{
                        Some(Token::Less)
                    }
                }
                '>' => {
                    self.forwardTokes();
                    if(self.currPosition()? == '='){
                        self.forwardTokes();
                        Some(Token::GreatEqual)
                    }else{
                        Some(Token::Great)
                    }
                }
                _ => {
                    
                    let identifier = self.identifiers();
                    if let Token::Identifier(ref id) = identifier {
                        match id.as_str() {
                            "println" => {
                                let currTab = self.tab;
                                self.tab = 0;
                                return Some(Token::kwPrint(currTab));
                            }
                            "int" => {
                                return Some(Token::kwInt);
                            }
                            "bool" =>{
                                return Some(Token::kwBool);
                            }
                            "null" => {
                                return Some(Token::Null);
                            }
                            "void" => {
                                return Some(Token::kwVoid);
                            }
                            "struct" => {
                                return Some(Token::kwStruct);
                            }
                            "func" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwFunc(currTab));
                            }
                            "break" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwBreak(currTab));
                            }
                            "return" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwReturn(currTab));
                            }
                            "if" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwIf(currTab));
                            }
                            "elif" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwElIf(currTab));
                            }
                            "else" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwElse(currTab));
                            }
                            "while" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwWhile(currTab));
                            }
                            "vardec" => {
                                let currTab = self.tab.clone();
                                self.tab = 0;
                                return Some(Token::kwVarDec(currTab));
                            }
                            "block" => {
                                return Some(Token::kwBlock);
                            }
                            "call" => {
                                return Some(Token::kwCall);
                            }
                            "new" => {
                                return Some(Token::kwNew);
                            }
                            "and" =>{
                                return Some(Token::And);
                            }
                            "or" =>{
                                return Some(Token::Or);
                            }
                            num if num.chars().all(|c| c.is_numeric()) => {
                                return Some(Token::Integer(num.parse().unwrap()));
                            }
                            _ => {
                                return Some(Token::Identifier(id.to_string()));   
                            }
                        }
                    }
                    Some(identifier)
                }
                _ => {
                    panic!("unrechognized token") // Unknown character, could return an error token or None
                }
            },
            None =>{ 
                self.eof_flag=true;
                return Some(Token::Eof);
                //process::exit(1)
            }, // End of input

        }
    }        
        
    
}
