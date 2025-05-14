mod parser;

use parser:Parser;
use crate::parser::ASTNode;
use crate::parser::Param;
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
        writeln!(self.file, "#include <stdio.h>");
        writeln!(self.file, "#include <stdlib.h>");
        writeln!(self.file, "#include <stdbool.h>");

        generate_garbage_collector();

        if let ASTNode::Program(nodes) = self.program.clone(){
            while self.pos < nodes.len(){
                match nodes[self.pos]{
                    ASTNode::StructDef(_, _, _) => {
                        self.generate_struct(nodes[self.pos]);
                    }
                    ASTNode::FuncDef(name, _, _) => {
                        if name == "main"{
                            self.generate_main();
                        else{
                            self.generate_func();
                        }
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

    pub fn generate_garbage_collector(&mut self){
        writeln!("typedef struct MarkRefs MarkRefs");
        writeln!("typedef struct Reference Reference;");
        writeln!("typedef struct Heap Heap;");

        writeln!("struct Reference{");
        writeln!("\tchar* object_lacation;");
        writeln!("\tsize_t object_size;");
        writeln!("\tMarkRefs children;");
        writeln!("\tbool allocated;");
        writeln!("\tbool mark;");
        writeln!("};\n");

        writeln!("struct Heap{");
        writeln!("\tchar* start;");
        writeln!("\tchar* mid;");
        writeln!("\tchar* bump-pointer;");
        writeln!("\tbool on-start;");
        writeln!("\tsize_t total_heap_size;");
        writeln!("\tsize_t current_heap_max;");
        writeln!("\tsize_t current_allocated_size;");
        writeln!("\tReference* entries;");
        writeln!("\tsize_t total_num_entries;");
        writeln!("};\n");
        
        writeln!("Reference gc_allocate(struct Heap* h, size_t s, MarkRefs mc){");
        writeln!("\tif (h.current_allocated_size + s <= h.current_heap_max){");
        writeln!("\t\tReference entry = {h.bump-pointer, s, mc, true, true}");
        writeln!("\t\th.current_allocated_size += s;");
        writeln!("\t\th.bump-pointer += s;");
        writeln!("}\n");
    }

    pub fn generate_struct(&mut self, structure: ASTNode){
        if let ASTNode::StructDef(name, parameters, body) = structure{
            writeln!(self.file, "typedef struct {");
            self.tab += 1;
            for param in parameters{
                let param_line = "";
                for x in 0..self.tab.clone(){
                    param_line += "\t";
                }
                
                if let Param(var_type, var) = param{
                    param_line += var_type + " " + var + ";"
                }

                writeln!(self.file, param_line);
            }

            for func in body{
                let func_line = "";
                for x in 0..self.tab.clone(){
                    func_line += "\t";
                }

                
            }
            writeln!(self.file, "} {};", name);
        } 
    }
    
    pub fn generate_func(&mut self){}
  
    pub fn generate_var(&mut self){}

    pub fn generate_stmt(&mut self){}

    pub fn generate_exp(&mut self){}

    pub fn generate_main(&mut self){
        writeln!(self.file, "typedef struct Node {\n\tvoid* data;\n\tstruct Node* next;\n} Node;");
        writeln!(self.file, "");
        writeln!(self.file, "int main() {");
        self.tab+=1;        
        
        self.tab-=1;
        writeln!(self.file, "}");
    }
}
