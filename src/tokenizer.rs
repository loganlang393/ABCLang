use crate::token::Token;
use std::process;


pub struct Tokenizer{
    tokes: Vec<char>, //Vec<char> cause I need indexing
    pos: usize,
    eof_flag: bool,
}

impl Tokenizer {
    pub fn newToken(tokes: &str) -> Self{
        Tokenizer {
            tokes: tokes.chars().collect(),// converts that string into vec<char>
            pos: 0, //position starts on 0
            eof_flag: false,
        }
    }
    
    // Returns index of token
    fn currPosition (&self) -> Option<char> {
        self.tokes.get(self.pos).copied()
    }

    //Skips whitespace
    fn skips (&mut self) {
        while let Some(curr) = self.currPosition(){
            if !curr.is_whitespace(){
                break;
            } 
            self.forwardTokes();
        }
        
    }

    //forwarder
    fn forwardTokes(&mut self) {
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
                    Some(Token::Equal)
                }
                ';' => {
                    self.forwardTokes();
                    Some(Token::Semicolon)
                }
                _ => {
                    
                    let identifier = self.identifiers();
                    if let Token::Identifier(ref id) = identifier {
                        match id.as_str() {
                            "println" => {
                                return Some(Token::kwPrint);
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
                                return Some(Token::kwFunc);
                            }
                            "break" => {
                                return Some(Token::kwBreak);
                            }
                            "return" => {
                                return Some(Token::kwReturn);
                            }
                            "if" => {
                                return Some(Token::kwIf);
                            }
                            "while" => {
                                return Some(Token::kwWhile);
                            }
                            "vardec" => {
                                return Some(Token::kwVarDec);
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
                            _ => {
                                return Some(Token::Identifier(id.to_string()));   
                            }
                        }
                    }
                    Some(identifier)
                }
                _ if curr.is_digit(10) => Some(self.readInteger()),
                _ if curr.is_alphanumeric() => Some(self.identifiers()),
                _ => {
                    self.forwardTokes(); // Skip over unrecognized characters
                    None // Unknown character, could return an error token or None
                }
            },
            None =>{ 
                self.eof_flag=true;
                self.forwardTokes();
                return Some(Token::Eof);
                //process::exit(1)
            }, // End of input

        }
    }        
        
    
}
