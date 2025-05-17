use crate::parser::{ASTNode, Param};
use crate::parser::Parser;
use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug)]
pub struct CodeGenerator {
    program: ASTNode,
    pos: usize,
    tab: usize,
    file: std::fs::File,
    heap: String,
    funcs: Vec<ASTNode>,
    strucs: Vec<ASTNode>,
}

impl CodeGenerator {
    pub fn new(mut parser: Parser, file_name: &str) -> Self {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("Failed to open output file");
        Self {
            program: parser.parse(),
            pos: 0,
            tab: 0,
            file,
            heap: "heap".to_string(),
            funcs: parser.funcs,
            strucs: parser.strucs,
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
                                self.generate_main(node);
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

        // MarkRefs struct, currently undeveloped as structures do not have the ability to store variables currently
        writeln!(self.file, "struct MarkRefs {{").unwrap();
        writeln!(self.file, "\tsize_t object_size;").unwrap();
        writeln!(self.file, "\tbool allocated;").unwrap();
        writeln!(self.file, "}};\n").unwrap();
        
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
        writeln!(self.file, "\tchar* total_heap_size;").unwrap();
        writeln!(self.file, "\tchar* current_heap_max;").unwrap();
        writeln!(self.file, "\tReference* entries;").unwrap();
        writeln!(self.file, "\tsize_t total_num_entries;").unwrap();
        writeln!(self.file, "\tsize_t num_entries;").unwrap();
        writeln!(self.file, "}};\n").unwrap();

        //gc_reallocate function
        writeln!(self.file, "void gc_reallocate(struct Heap* h) {{").unwrap();
        writeln!(self.file, "\tif(h->on_start) {{").unwrap();
        writeln!(self.file, "\t\th->on_start = false;").unwrap();
        writeln!(self.file, "\t\th->bump_pointer = h->mid;").unwrap();
        writeln!(self.file, "\t}}else {{").unwrap();
        writeln!(self.file, "\t\th->on_start = true;").unwrap();
        writeln!(self.file, "\t\th->bump_pointer = h->start;").unwrap();
        writeln!(self.file, "\t}}").unwrap();
        writeln!(self.file, "\tReference* current_list = h->entries;").unwrap();
        writeln!(self.file, "\th->entries = malloc(sizeof(Reference) * h->total_num_entries);").unwrap();
        writeln!(self.file, "\tfor(size_t i = 0; i < h->total_num_entries; i++) {{").unwrap();
        writeln!(self.file, "\t\tif(current_list[i]->mark) {{").unwrap();
        writeln!(self.file, "\t\t\th->entries[h->num_entries] = *gc_allocate(h, current_list[i]->object_size, current_list[i]->children);").unwrap();
        writeln!(self.file, "\t\t\tfree(current_list[i]);").unwrap();
        writeln!(self.file, "\t\t\tcurrent_list[i] = NULL").unwrap();
        writeln!(self.file, "\t\t}}else {{").unwrap();
        writeln!(self.file, "\t\t\tfree(current_list[i]);").unwrap();
        writeln!(self.file, "\t\t\tcurrent_list[i] = NULL;").unwrap();
        writeln!(self.file, "\t\t}}").unwrap();
        writeln!(self.file, "\t}}").unwrap();
        writeln!(self.file, "\tfree(current_list);").unwrap();
        writeln!(self.file, "\tcurrent_list = NULL").unwrap();
        writeln!(self.file, "}}\n").unwrap();

        // gc_allocate function
        writeln!(self.file, "Reference gc_allocate(struct Heap* h, size_t s, MarkRefs* mc) {{").unwrap();
        writeln!(self.file, "\tif(h->on_start) {{").unwrap();

        writeln!(self.file, "\t\tif (h->bump_pointer + s <= h->mid) {{").unwrap();

        writeln!(self.file, "\t\t\tif (h->num_entries == h->total_num_entries) {{").unwrap();
        writeln!(self.file, "\t\t\t\tgc_reallocate(&h);").unwrap();

        writeln!(self.file, "\t\t\t\tif (h->num_entries == h->total_num_entries) {{").unwrap();
        writeln!(self.file, "\t\t\t\t\tRefernce entry = {{h->bump_pointer, s, mc, false, false}};").unwrap();
        writeln!(self.file, "\t\t\t\t\treturn entry;").unwrap();

        writeln!(self.file, "\t\t\t\t}} else {{").unwrap();
        writeln!(self.file, "\t\t\t\t\treturn gc_allocate(&h, s, &mc);").unwrap();
        writeln!(self.file, "\t\t\t\t}}").unwrap();
        writeln!(self.file, "\t\t\t}}").unwrap();

        writeln!(self.file, "\t\t\tReference entry = {{h->bump_pointer, s, mc, true, true}};").unwrap();
        writeln!(self.file, "\t\t\th->entries[num_entries] = entry;").unwrap();
        writeln!(self.file, "\t\t\th->num_entries++;").unwrap();
        writeln!(self.file, "\t\t\th->bump_pointer += s;").unwrap();
        writeln!(self.file, "\t\t\treturn entry;").unwrap();

        writeln!(self.file, "\t\t}} else {{").unwrap();
        writeln!(self.file, "\t\t\tgc_reallocate(&h);").unwrap();
        writeln!(self.file, "\t\t\treturn gc_allocate(&h, s, &mc);").unwrap();
        writeln!(self.file, "\t\t}}").unwrap();

        writeln!(self.file, "\t}} else {{").unwrap();

        writeln!(self.file, "\t\tif (h->bump_pointer + s <= h->total_heap_size) {{").unwrap();

        writeln!(self.file, "\t\t\tif (h->num_entries == h->total_num_entries) {{").unwrap();
        writeln!(self.file, "\t\t\t\tgc_reallocate(&h);").unwrap();

        writeln!(self.file, "\t\t\t\tif (h->num_entries == h-> total_num_entries) {{").unwrap();
        writeln!(self.file, "\t\t\t\t\tReference entry = {{h->bump_pointer, s, mc, false, false}};").unwrap();
        writeln!(self.file, "\t\t\t\t\treturn NULL;").unwrap();

        writeln!(self.file, "\t\t\t\t}} else {{").unwrap();
        writeln!(self.file, "\t\t\t\t\treturn gc_allocate(&h, s, &mc);").unwrap();
        writeln!(self.file, "\t\t\t\t}}").unwrap();
        writeln!(self.file, "\t\t\t}}").unwrap();

        writeln!(self.file, "\t\t\tReference entry = {{h->bump_pointer, s, mc, true, true}};").unwrap();
        writeln!(self.file, "\t\t\th->num_entries++;").unwrap();
        writeln!(self.file, "\t\t\th->bump_pointer += s;").unwrap();
        writeln!(self.file, "\t\t\treturn &entry;").unwrap();

        writeln!(self.file, "\t\t}} else {{").unwrap();
        writeln!(self.file, "\t\t\tgc_reallocate(&h);").unwrap();
        writeln!(self.file, "\t\t\treturn gc_allocate(&h, s, &mc);").unwrap();
        writeln!(self.file, "\t\t}}").unwrap();
        writeln!(self.file, "\t}}").unwrap();
        writeln!(self.file, "}}").unwrap();

	// gc_deallocate function
	writeln!(self.file, "void gc_deallocate(struct Heap* h, Reference reference) {{").unwrap();
        writeln!(self.file, "\th->current_allocated_size -= reference->object_size;").unwrap();
        writeln!(self.file, "\treference->mark=false;").unwrap();
        writeln!(self.file, "}}").unwrap();
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
                    let mut func_params = params.clone();
                    func_params.insert(0, Param {var_type: format!("{}*", name), var: format!("&s")});
                    self.generate_func(ASTNode::FuncDef(func_name.clone(), func_params.clone(), ret_type.clone(), body.clone()));
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

            let mut func_structs = Vec::new();

            self.tab += 1;
            for stmt in body {
                if let ASTNode::VarDec(_, var_type, _) = stmt.clone(){
                    if var_type != "int".to_string() || var_type != "bool".to_string(){
                        func_structs.push(var_type.clone().to_string());
                        writeln!(self.file, "{}Reference {}_reference = gc_allocate(&{}, sizeof({}), NULL);", indent, var_type.clone(), self.heap, var_type.clone()).unwrap();
                    }
                }
                self.generate_stmt(stmt.clone());
            }

            for reference in func_structs{
                writeln!(self.file, "{}gc_deallocate(&{}, {}_reference);",indent, self.heap, reference.to_string());
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
                    for else_stmts in &else_branch.clone(){
                        match else_stmts{
                            ASTNode::ElIf(cond, elif_body) => {
                                write!(self.file, "{}else if (", indent).unwrap();
                                self.generate_exp(&*cond);
                                writeln!(self.file, ") {{").unwrap();
                                self.tab += 1;
                                for stmt in elif_body {
                                    self.generate_stmt(stmt.clone());
                                }
                                self.tab -= 1;
                                writeln!(self.file, "{}}}", indent).unwrap();
                            }
                            ASTNode::Else(else_body) => {
                                writeln!(self.file, "{}else {{", indent).unwrap();
                                self.tab += 1;
                                for stmt in else_body {
                                    self.generate_stmt(stmt.clone());
                                }
                                self.tab -= 1;
                                writeln!(self.file, "{}}}", indent).unwrap();
                            }
                            _ =>{
                                panic!("Only else or elif statements can be generated");
                            }
                        }
                    }
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
            ASTNode::Collect =>{
                writeln!(self.file, "gc_reallocate(&{});", self.heap).unwrap();
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
            ASTNode::Struct(name, params) => {
                write!(self.file, "{{"). unwrap();   

                for x in params{
                    self.generate_exp(x)
                }
                write!(self.file, "}}").unwrap();
            }
            ASTNode::Func(name, params) => {
                write!(self.file, "{}(", name).unwrap();
                
                for x in params{
                    self.generate_exp(x);
                }
                write!(self.file, ")").unwrap();
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

    fn generate_main(&mut self, node: ASTNode) {
        let indent = "\t";
        writeln!(self.file, "int main() {{").unwrap();
        self.tab += 1;
        writeln!(self.file, "{}char* heap_list = malloc(sizeof(char) * 1024);", indent).unwrap();
        writeln!(self.file, "{}Heap {} = {{heap_list, &heap_list[511], &heap_list[0], true, &heap_list[1023], 1024, malloc(sizeof(Reference) * 50), 0}}", indent, self.heap).unwrap();
             
        // Generate main body here or leave empty
        if let ASTNode::FuncDef(_, _, _, body) = node {
            let mut func_structs = Vec::new();

            for stmt in body {                                                                                                          
                if let ASTNode::VarDec(_, var_type, _) = stmt.clone(){                                                                      
                    if var_type != "int".to_string() || var_type != "bool".to_string(){                                                         
                        func_structs.push(var_type.clone().to_string());                                                                        
                        writeln!(self.file, "{}Reference {}_reference = gc_allocate(&{}, sizeof({}), NULL);", indent, var_type.clone(), self.heap, var_type.clone());
                    }
                }    
                self.generate_stmt(stmt.clone());
            }

            for reference in func_structs{
                writeln!(self.file, "{}gc_deallocate(&{}, {}_reference);",indent, self.heap, reference.to_string());
            }
        }
        self.tab -= 1;
        writeln!(self.file, "}}").unwrap();
    }
}
