
mod token;
use token::*;


pub struct tokenizer{
    tokes: Vec<char>, //Vec<char> cause I need indexing
    posi: usize,
}

impl tokenizer {
    pub fn newToken(nToken: &str) -> Self{
        tokenizer {
            tokes: nToken.chars().collect(),// converts that string into vec<char>
            posi: 0, //position starts on 0
        }
    }
    
    // Returns index of token
    fn currPosition (&self) -> Option<char> {
        self.tokes.get(self.posi).copied()
    }

    //Skips whitespace
    fn skips (&mut self) {
        let curr = self.currPosition();
        while self.posi < self.tokes.len() {
            if !curr.expect("REASON").is_whitespace(){
                break;
            } 
            self.forwardTokes();
        }
        
    }

    //forwarder
    fn forwardTokes(&mut self) -> Option<char> {
        let curr = self.currPosition();
        self.posi += 1;
        curr
        
    }

    fn readInteger(&mut self) {
        let mut digit = String::new();

        while let Some(curr) = self.currPosition() {
            if curr.is_digit(10){
                digit.push(curr);
                self.forwardTokes();
            } else{
                break;
            }
        }
        
    }

    fn readString(&mut self) {
        let mut string = String::new();
        while let Some(curr) = self.currPosition() {
            string.push(curr);
            self.forwardTokes();
        }
    }

    fn identifiers(&mut self) -> String{
        let mut identifier = String::new();
        while let Some(curr) = self.currPosition(){
            if curr.is_alphanumeric() { //identifiers only characters and number)
                identifier.push(curr);
                self.forwardTokes();
                
            }
        }
        identifier
    }
    //Still work in progress
    pub fn readToken(&mut self) {
        self.skips();
        match self.currPosition(){
            Some(curr) => match curr {
                '(' => {
                    self.forwardTokes();
                }
                ')' => {
                    self.forwardTokes();
                }
                '+' => {
                    self.forwardTokes();
                    
                }
                '-' => {
                self.forwardTokes();
                }
                '*' => {
                    self.forwardTokes();
                }
                '/' => {
                    self.forwardTokes();
                }
                '=' => {
                    self.forwardTokes();
                }
                ';' => {
                    self.forwardTokes();
                }

            curr if curr.is_digit(10) => self.readInteger(),

           curr if curr.is_alphanumeric() => {
                let identifier = self.identifiers();
                match identifier.as_str(){
                    "print" => {
                        self.forwardTokes();
                        //print token
                    },
                    &_ => todo!()
                }
            },
            _ => todo!()
                
            },
            None => todo!()
        }
    }
}
