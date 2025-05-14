use crate::parser::{ASTNode, Param};
use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug)]
pub struct CodeGenerator {
    program: ASTNode,
    pos: usize,
    tab: usize,
    file: std::fs::File,
}

impl CodeGenerator {
    pub fn new(ast: ASTNode, file_name: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("Failed to open output file");
        Self {
            program: ast,
            pos: 0,
            tab: 0,
            file,
        }
    }

    pub fn generate(&mut self) {
        // Write includes
        writeln!(self.file, "#include <stdio.h>").unwrap();
        writeln!(self.file, "#include <stdlib.h>").unwrap();
        writeln!(self.file, "#include <stdbool.h>").unwrap();
        writeln!(self.file).unwrap();

        // Generate garbage collector support code
        self.generate_garbage_collector();

        // Check if program is a `Program` node
        if let ASTNode::Program(nodes) = &self.program {
            // Clone nodes to avoid borrow conflicts
            let nodes = nodes.clone();
            for node in nodes {
                match node {
                    ASTNode::StructDef(_, _, _) => {
                        self.generate_struct(node);
                    }
                    ASTNode::FuncDef(_, _, _, _) => {
                        if let ASTNode::FuncDef(ref name, _, _, _) = node {
                            if name == "main" {
                                self.generate_main();
                            } else {
                                self.generate_func(node);
                            }
                        }
                    }
                    ASTNode::VarDec(_, _, _) => {
                        self.generate_var_dec(node);
                    }
                    _ => {
                        self.generate_stmt(node);
                    }
                }
            }
        }
    }
        
    

    fn generate_garbage_collector(&mut self) {
        // Basic GC structs
        writeln!(self.file, "typedef struct MarkRefs MarkRefs;").unwrap();
        writeln!(self.file, "typedef struct Reference Reference;").unwrap();
        writeln!(self.file, "typedef struct Heap Heap;\n").unwrap();

        // Reference struct
        writeln!(self.file, "struct Reference {{").unwrap();
        writeln!(self.file, "\tchar* object_location;").unwrap();
        writeln!(self.file, "\tsize_t object_size;").unwrap();
        writeln!(self.file, "\tMarkRefs* children;").unwrap();
        writeln!(self.file, "\tbool allocated;").unwrap();
        writeln!(self.file, "\tbool mark;").unwrap();
        writeln!(self.file, "}};\n").unwrap();

        // Heap struct
        writeln!(self.file, "struct Heap {{").unwrap();
        writeln!(self.file, "\tchar* start;").unwrap();
        writeln!(self.file, "\tchar* mid;").unwrap();
        writeln!(self.file, "\tchar* bump_pointer;").unwrap();
        writeln!(self.file, "\tbool on_start;").unwrap();
        writeln!(self.file, "\tsize_t total_heap_size;").unwrap();
        writeln!(self.file, "\tsize_t current_heap_max;").unwrap();
        writeln!(self.file, "\tsize_t current_allocated_size;").unwrap();
        writeln!(self.file, "\tReference* entries;").unwrap();
        writeln!(self.file, "\tsize_t total_num_entries;").unwrap();
        writeln!(self.file, "}};\n").unwrap();

        // gc_allocate function
        writeln!(self.file, "Reference gc_allocate(struct Heap* h, size_t s, MarkRefs* mc) {{").unwrap();
        writeln!(self.file, "\tif (h->current_allocated_size + s <= h->current_heap_max) {{").unwrap();
        writeln!(self.file, "\t\tReference entry = {{h->bump_pointer, s, mc, true, true}};").unwrap();
        writeln!(self.file, "\t\th->current_allocated_size += s;").unwrap();
        writeln!(self.file, "\t\th->bump_pointer += s;").unwrap();
        writeln!(self.file, "\t\treturn entry;").unwrap();
        writeln!(self.file, "\t}}").unwrap();
    }

    fn generate_struct(&mut self, structure: ASTNode) {
        if let ASTNode::StructDef(name, parameters, body) = structure {
            let indent = "\t";
            writeln!(self.file, "typedef struct {} {{", name).unwrap();
            self.tab += 1;
            for param in parameters {
                if let Param{var_type, var} = param {
                    let indentation = "\t".repeat(self.tab);
                    writeln!(self.file, "{}{} {};", indentation, var_type, var).unwrap();
                }
            }
            self.tab -= 1;
            writeln!(self.file, "}} {};\n", name).unwrap();

            // Generate embedded functions if any
            for func in body {
                if let ASTNode::FuncDef(func_name, params, ret_type, body) = func {
                    self.generate_func(ASTNode::FuncDef(func_name.clone(), params.clone(), ret_type.clone(), body.clone()));
                }
            }
        }
    }

    fn generate_func(&mut self, node: ASTNode) {
        if let ASTNode::FuncDef(name, params, ret_type, body) = node {
            let params_str = params
                .iter()
                .map(|p| format!("{} {}", p.var_type, p.var))
                .collect::<Vec<_>>()
                .join(", ");
            let indent = "\t";

            writeln!(self.file, "{}{} {}({}) {{", indent, ret_type, name, params_str).unwrap();

            self.tab += 1;
            for stmt in body {
                self.generate_stmt(stmt.clone());
            }
            self.tab -= 1;
            writeln!(self.file, "{}}}\n", indent).unwrap();
        }
    }

    fn generate_var_dec(&mut self, node: ASTNode) {
        if let ASTNode::VarDec(var_name, var_type, expr) = node {
            let indent = "\t".repeat(self.tab);
            write!(self.file, "{}{} {} = ", indent, var_type, var_name).unwrap();
            self.generate_exp(&*expr);
            writeln!(self.file, ";").unwrap();
        }
    }

    fn generate_stmt(&mut self, node: ASTNode) {
        let indent = "\t".repeat(self.tab);
        match node {
            ASTNode::Return(Some(expr)) => {
                write!(self.file, "{}return ", indent).unwrap();
                self.generate_exp(&*expr);
                writeln!(self.file, ";").unwrap();
            }
            ASTNode::Return(None) => {
                writeln!(self.file, "{}return;", indent).unwrap();
            }
            ASTNode::VarDec(_, _, _) => {
                self.generate_var_dec(node);
            }
            ASTNode::If(cond, then_branch, else_branch) => {
                write!(self.file, "{}if (", indent).unwrap();
                self.generate_exp(&*cond);
                writeln!(self.file, ") {{").unwrap();
                self.tab += 1;
                for stmt in then_branch {
                    self.generate_stmt(stmt.clone());
                }
                self.tab -= 1;
                writeln!(self.file, "{}}}", indent).unwrap();
                if !else_branch.is_empty() {
                    writeln!(self.file, "{}else {{", indent).unwrap();
                    self.tab += 1;
                    for stmt in else_branch {
                        self.generate_stmt(stmt.clone());
                    }
                    self.tab -= 1;
                    writeln!(self.file, "{}}}", indent).unwrap();
                }
            }
            ASTNode::While(cond, body) => {
                write!(self.file, "{}while (", indent).unwrap();
                self.generate_exp(&*cond);
                writeln!(self.file, ") {{").unwrap();
                self.tab += 1;
                for stmt in body {
                    self.generate_stmt(stmt.clone());
                }
                self.tab -= 1;
                writeln!(self.file, "{}}}", indent).unwrap();
            }
            ASTNode::Print(expr) => {
                write!(self.file, "{}printf(\"%d\\n\", ", indent).unwrap();
                self.generate_exp(&*expr);
                writeln!(self.file, ");").unwrap();
            }
            ASTNode::Set(stmts) => {
                if !stmts.is_empty() {
                    if let ASTNode::Var(var_name) = &stmts[0] {
                        write!(self.file, "{}{} = ", indent, var_name).unwrap();
                        self.generate_exp(&stmts[1]);
                        writeln!(self.file, ";").unwrap();
                    } else if let ASTNode::PropertyAccess(obj, prop) = &stmts[0] {
                        // e.g., (. first next) -> first->next
                        write!(self.file, "{}{:?}->{} = ", indent, obj, prop).unwrap();
                        self.generate_exp(&stmts[1]);
                        writeln!(self.file, ";").unwrap();
                    }
                }
            }
            _ => {
                // Handle other statements if needed
            }
        }
    }

    fn generate_exp(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Integer(n) => {
                write!(self.file, "{}", n).unwrap();
            }
            ASTNode::Bool(b) => {
                write!(self.file, "{}", if *b { "true" } else { "false" }).unwrap();
            }
            ASTNode::Var(name) => {
                write!(self.file, "{}", name).unwrap();
            }
            ASTNode::AddOrMinusExp(parts) => {
                if parts.len() >= 3 {
                    self.generate_exp(&parts[1]);
                    let op = match parts[0] {
                        ASTNode::AddOp => "+",
                        ASTNode::MinusOp => "-",
                        _ => "?",
                    };
                    write!(self.file, " {} ", op).unwrap();
                    self.generate_exp(&parts[2]);
                }
            }
            ASTNode::MultOrDivExp(parts) => {
                if parts.len() >= 3 {
                    self.generate_exp(&parts[1]);
                    let op = match parts[0] {
                        ASTNode::MultOp => "*",
                        ASTNode::DivOp => "/",
                        _ => "?",
                    };
                    write!(self.file, " {} ", op).unwrap();
                    self.generate_exp(&parts[2]);
                }
            }
            ASTNode::CompExp(parts) => {
                if parts.len() == 3 {
                    self.generate_exp(&parts[1]);
                    let op = match &parts[0] {
                        ASTNode::CompOp(op_str) => op_str.as_str(),
                        _ => "?",
                    };
                    write!(self.file, " {} ", op).unwrap();
                    self.generate_exp(&parts[2]);
                }
            }
            ASTNode::AndExp(parts) => {
                if parts.len() == 3 {
                    self.generate_exp(&parts[1]);
                    write!(self.file, " && ").unwrap();
                    self.generate_exp(&parts[2]);
                }
            }
            ASTNode::OrExp(parts) => {
                if parts.len() == 3 {
                    self.generate_exp(&parts[1]);
                    write!(self.file, " || ").unwrap();
                    self.generate_exp(&parts[2]);
                }
            }
            ASTNode::NotExp(parts) => {
                if parts.len() == 2 {
                    write!(self.file, "!").unwrap();
                    self.generate_exp(&parts[1]);
                }
            }
            ASTNode::Call(func_name, args) => {
                write!(self.file, "{}(", func_name).unwrap();
                for (i, arg) in args.iter().enumerate() {
                    self.generate_exp(arg);
                    if i < args.len() - 1 {
                        write!(self.file, ", ").unwrap();
                    }
                }
                write!(self.file, ")").unwrap();
            }
            ASTNode::PropertyAccess(obj, prop) => {
                // e.g., (. first next) -> first->next
                write!(self.file, "{:?}->{}", obj, prop).unwrap();
            }
            ASTNode::Null => {
                write!(self.file, "NULL").unwrap();
            }
            _ => {
                // Fallback
                write!(self.file, "/* unsupported expr */").unwrap();
            }
        }
    }

    fn generate_main(&mut self) {
        let indent = "\t";
        writeln!(self.file, "int main() {{").unwrap();
        self.tab += 1;
        // Generate main body here or leave empty
        self.tab -= 1;
        writeln!(self.file, "{}}}", indent).unwrap();
    }
}
