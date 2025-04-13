//ParseResult
//expressions
//operations
//statements

use std::any::Any;

pub struct ParseResult{
   result: Box<dyn Any>,
   position: i32,
}

impl ParseResult{
   fn new<T: 'static' + Any>(value: T, pos: i32) -> Self{
      Self {
         result: Box::new(value),
         pos,
      }
   }
}


//Exp
//IdExp
//IntExp
pub enum Exp {
   IdExp(String),
   IntExp(i32),
}

//Op
//MinusOp
//AddOp
//MultOp
//DivOp
//ExpOp
pub enum Op{
   MinuxOp,
   AddOp,
   MultOp,
   DivOp,
}

//Stmt
//ReturnStmt
//PrintStmt
pub enum Stmt{
   ReturnStmt,
   PrintStmt,
}
