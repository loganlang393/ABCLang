use crate::token::Token;
mod parseResult;
use std::process;

pub struct Parser{
   tokens:Vec<Token>,
}

impl Parser{
   pub new(&mut self, list: Vec<Token>){
      self.tokens = list;
   }

   pub fn readToken(&mut self, position: i32) -> Result<Token, &'static' str>{
      if(position < 0 || position > self.tokens.len()){
         Err("Index Out Of Bounds")
      }
      else{
         if match!(self.tokens[position], Tokens::EoF){
            Err("End of File")
         }else{
            Ok(self.tokens[position])
         }
      }
   }

   pub fn primaryExp(&self, position: i32) -> Result<ParseResult, &'static' str>{
      let t = self.readToken(Position);
      match t {
         Token::Identifier(name) => return Ok(ParseResult::new(IdExp(name), position + 1));
         Token::Integer(num) => return Ok(ParseResult::new(IntExp(num), position + 1));
         Token::lParen => {
            let e = ParseResult(
         }
      }
   }

   pub fn multExp(position: i32) -> ParseResult{

   }

   pub fn divExp(position: i32) -> ParseResult{

   }

   pub fn addExp(position: i32) -> ParseResutl{

   }

   pub fn exp(position: i32) -> ParseResult{

   }

   pub fn stmt(position: i32) -> ParseResult{

   }
}


