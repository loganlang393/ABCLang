mod parser;

use parser:Parser;
use crate::parser::ASTNode;
use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug)]
pub struct CodeGenerator{
    program: ASTNode,
    pos: usize,
    tab: i32,
    file: std::fs::File 
}

impl CodeGenerator{
    pub fn new(mut parser: Parser, file_name: &str) -> Self{
        CodeGenerator {parser.parse(), pos: 0, tab: 0, OpenOptions::new().append(true).create(true).open(file_name)}
    }

    pub fn generate(&mut self){
        writeln!(self.file, "include <stdio.h>");
        writeln!(self.file, "#include<string.h>");

        if let ASTNode::Program(nodes) = self.program.clone(){
            while self.pos < nodes.len(){
                match nodes[self.pos]{
                    ASTNode::StructDef(_, _, _) => {
                        self.generate_struct(nodes[self.pos]);
                    }
                    ASTNode::FuncDef(_, _, _) => {
                        self.generate_func();
                    }
                    ASTNode::VarDec(_, _, _) => {
                        self.generate_var();
                    }
                    _ => {
                        self.generate_stmt();
                    }
                }
            }
        }
    }

    pub fn generate_struct(&mut self, structure: ASTNode){
        if let ASTNode::StructDef(name, parameters, body) = structure{
            writeln!(self.file, "typedef struct {");
            writeln!(self.file, "} {};", name);
        } 
    }
    
    pub fn generate_func(&mut self){}
  
    pub fn generate_var(&mut self){}

    pub fn generate_stmt(&mut self){}

    pub fn generate_exp(&mut self){}
}
