// parser.rs
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::process;

#[derive(Debug)]
#[derive(Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    StructDef(String, Vec<Param>, Vec<ASTNode>),
    FuncDef(String, Vec<Param>, String, Vec<ASTNode>),
    VarDec(String, String, Box<ASTNode>),
    Var(String),
    Func(String, Vec<ASTNode>),
    Struct(String, Vec<ASTNode>),
    Assignment(Box<ASTNode>, Box<ASTNode>),
    PropertyAccess(Box<ASTNode>, String),
    If(Box<ASTNode>, Vec<ASTNode>, Vec<ASTNode>),
    ElIf(Box<ASTNode>, Vec<ASTNode>),
    Else(Vec<ASTNode>),
    While(Box<ASTNode>, Vec<ASTNode>),
    Print(Box<ASTNode>),
    Return(Option<Box<ASTNode>>),
    Block(Vec<ASTNode>),
    Set(Vec<ASTNode>),
    Integer(i32),
    Bool(bool),
    Null,
    Call(String, Vec<ASTNode>),
    New(String, Vec<ASTNode>),
    LHS(String),
    AddOrMinusExp(Vec<ASTNode>),
    MultOrDivExp(Vec<ASTNode>),
    AndExp(Vec<ASTNode>),
    OrExp(Vec<ASTNode>),
    NotExp(Vec<ASTNode>),
    CompExp(Vec<ASTNode>),
    AddOp,
    MinusOp,
    MultOp,
    DivOp,
    AndOp,
    OrOp,
    NotOp,
    CompOp(String),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Param {
    pub var_type: String,
    pub var: String, 
}

pub struct Parser {
    tokens: Vec<Token>,
    pub funcs: Vec<ASTNode>,
    pub strucs: Vec<ASTNode>,
    pos: usize,
    tab: i32,
}

impl Parser {
    pub fn new(mut tokenizer: Tokenizer) -> Self{
        Parser { tokens: tokenizer.tokenize(), funcs: Vec::new(), strucs: Vec::new(), pos: 0, tab:0 }
    }

    pub fn parse(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
		
        
        while self.pos < self.tokens.len() {
            match Some(self.tokens[self.pos].clone()) {
                Some(Token::kwStruct) => {
                    self.pos+=1;
                    nodes.push(self.parse_struct_def());
                }
                Some(Token::kwFunc(tab)) => {
                    self.pos+=1;
                    nodes.push(self.parse_func_def(tab));
                }
                Some(Token::kwVarDec(tab)) => {
                    self.pos+=1;
                    nodes.push(self.parse_var_dec(tab));
                }
                Some(Token::Eof) =>{
                    self.pos+=1;
                    println!("End of file");
                    break;
                }
                _ => {
                    // literally everything else
                    if let Some(node) = self.parse_stmt() {
                        nodes.push(node);
                    }else{
                        panic!("Can't parse {}", self.tokens[self.pos].clone().toString());
                    }
                }
            }
        }

        ASTNode::Program(nodes)
    }

    
// breaks (panics!)
    fn parse_struct_def(&mut self) -> ASTNode {
        if self.tab > 0 {
            panic!("Failed to parse struct definition: can't nest structure")
        }

        println!("Parsing that structure!");
        if let Token::Identifier(name) = self.tokens[self.pos].clone() {
            self.pos+=1;
            println!("Struct name: {}", name);
            let mut params = Vec::new();
            if let Token::lParen = self.tokens[self.pos].clone() {
                self.pos+=1;
                println!("while loop");
                while let Some(param) = self.parse_param() {
                    println!("Before Push");
                    params.push(param);
                }
                if matches!(self.tokens[self.pos].clone(), Token::rParen) {
                    self.tab += 1;
                    self.pos+=1;
                    let mut body = Vec::new();

                    while let stmt = self.tokens[self.pos].clone(){
                        println!("loop structure {}", stmt.clone().toString());
                        match stmt {
                            Token::kwFunc(tab) => {
                                //println!("func");
                                if(tab == self.tab){
                                    self.pos+=1;
                                    body.push(self.parse_func_def(tab));
                                }else{
                                    break;
                                }
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    self.tab -= 1;
                    self.strucs.push(ASTNode::StructDef(name.clone(), params.clone(), body.clone()));
                    return ASTNode::StructDef(name, params, body);
                }else{
                    panic!("Failed to parse struct definition: no right parenthisis for parameters");
                }
            }else{
                panic!("Failed to parse struct definition: no left paranthesis for parameters");
            }
        }else{
            panic!("Failed to parse struct definition: not labeled");
        }
    }

    fn parse_param(&mut self) -> Option<Param> {
        let param_type = match self.tokens[self.pos].clone() {
            Token::kwInt => "int".to_string(),
            Token::kwBool => "bool".to_string(),
            Token::Identifier(var) => var,
            _ => return None,
        };

        self.pos+=1;

        if let Token::Identifier(var_name) = self.tokens[self.pos].clone() {
            self.pos+=1;
            return Some(Param {var_type: param_type, var: var_name});
        }else{
            panic!("no parameter identifier");
        }
    }
    
    

    fn parse_func_def(&mut self, tab: i32) -> ASTNode {
        if let Token::Identifier(name) = self.tokens[self.pos].clone() {
            self.pos+=1;
            let mut params = Vec::new();
            println!("{}", name);
            //self.tokenizer.forwardTokes();
            if let Token::lParen = self.tokens[self.pos].clone() {
                self.pos+=1;
                while let Some(param) = self.parse_param() {
                    params.push(param);
                }
                if let Token::rParen = self.tokens[self.pos].clone() {
                    self.pos+=1;
                    let ret_type = match self.tokens[self.pos].clone() {
                        Token::kwVoid => "void".to_string(),
                        Token::kwInt => "int".to_string(),
                        Token::kwBool => "bool".to_string(),
                        _ => panic!("Expected return type"),
                    };
                    
                    self.pos+=1;
                    self.tab += 1;
                    let mut body = Vec::new();
                    while let Some(stmt) = self.parse_stmt(){
                        println!("loop function ");
                        body.push(stmt);
                    }
                    self.tab -= 1;
                    //println!("{}", self.tab);
                    self.funcs.push(ASTNode::FuncDef(name.clone(), params.clone(), ret_type.clone(), body.clone()));
                    return ASTNode::FuncDef(name, params, ret_type, body);
                }else{
                    panic!("Failed to parse function definition: no right parenthesis");
                }
            }else{
                panic!("Failed to parse funcion definition: no left parenthesis");
            }
        }else{
            panic!("Failed to parse function definition: not named");
        }
    }

    fn parse_var_dec(&mut self, tab: i32) -> ASTNode {
        let param_type = match self.tokens[self.pos].clone() {
            Token::kwInt => "int".to_string(),
            Token::kwBool => "bool".to_string(),
            Token::Identifier(name) => {
                let mut var_type = "".to_string();
                for x in 0..self.strucs.len(){
                    if let ASTNode::StructDef(struct_name, _, _) = &self.strucs[x]{
                            if name == *struct_name{
                                var_type = name.to_string();
                            }
                        }
                }
                if var_type == ""{
                    panic!("Unknown variable type {}", name)
                }
                var_type
            }
            _ => panic!("Expected variable type"),
        };
        self.pos+=1;

        if let Token::Identifier(var_name) = self.tokens[self.pos].clone() {
            self.pos+=1;
            if let Some(exp) = self.parse_exp() {
                return ASTNode::VarDec(var_name, param_type, Box::new(exp));
            }else{
                panic!("Failed to parse variable declaration");
            }
        }else{
            panic!("Failed to parse variable declaration");
        }
    }

// This one actually works, just dies when doing the other parsing
    fn parse_stmt(&mut self) -> Option<ASTNode> {
        let token = self.tokens[self.pos].clone();
        println!("statement {}", token.clone().toString());
        match token {
            Token::kwReturn(tab) => {
                if(tab == self.tab){
                    return Some(self.parse_return(tab));
                }else{
                    return None;
                }
            }
            Token::kwIf(tab) => {
                if(tab == self.tab){
                    return Some(self.parse_if());
                }else{
                    return None;
                }
            }
            Token::kwWhile(tab) => {
                if(tab == self.tab){
                    return Some(self.parse_while(tab));
                }else{
                    return None;
                }
            }
            Token::kwPrint(tab) => {
                if(tab == self.tab){
                    return Some(self.parse_print(tab));
                }else{
                    return None;
                }
            }
            Token::kwSet(tab) => {
                if(tab == self.tab){
                    return Some(self.parse_set());
                }else{
                    return None;
                }
            }
            // Add other statements as necessary
            _ => {return None}
        }
    }

    fn parse_return(&mut self, tab: i32) -> ASTNode {
        let mut exp = None;
        self.pos+=1;
        if let token = self.tokens[self.pos].clone() {
            if token != Token::rParen {
                exp = Some(Box::new(self.parse_exp().expect("Expected expression")));
            }else{
                panic!("Can't parse return statement")
            }
        }
        ASTNode::Return(exp)
    }

    fn parse_if(&mut self) -> ASTNode {
        self.pos+=1;

        let condition;
        if let Token::lParen = self.tokens[self.pos].clone() {
            self.pos+=1;
            condition = self.parse_exp().expect("Failed to parse if statment: expected condition for if statement");
            
            if let Token::rParen = self.tokens[self.pos].clone(){
                self.pos+=1;
            }else{
                panic!("Failed to parse if statemnet: missing right parenthesis for conditional");
            }
        }else{
            panic!("Failed to parse if statment: missing left parenthisis for conditional");
        }

        let mut then_branch = Vec::new();
	
	self.tab+=1;
        while let Some(stmt) = self.parse_stmt() {
            then_branch.push(stmt);
        }
        self.tab-=1;

        let mut elseStmts = Vec::new();
            
        while let Token::kwElIf(tab) = self.tokens[self.pos].clone(){
            if(tab == self.tab){
                elseStmts.push(self.parse_elif());
            }
        }

        if let Token::kwElse(tab) = self.tokens[self.pos].clone(){
            if(tab == self.tab){
                elseStmts.push(self.parse_else());
            }
        }

        return ASTNode::If(Box::new(condition), then_branch, elseStmts);
    }

    fn parse_elif(&mut self) -> ASTNode {
        self.pos += 1;

        let condition;
        if let Token::lParen = self.tokens[self.pos].clone(){
            self.pos+=1;
            condition = self.parse_exp().expect("Failed to parse elif statement: missing condition");

            if let Token::rParen = self.tokens[self.pos].clone(){
                self.pos+=1;
            }else{
                panic!("Failed to parse elif statement: missing right parenthesis for condition");
            }
        }else{
            panic!("Failed to parse elif statement: missing left parenthesis for condition");
        }
        let mut then_branch = Vec::new();

        self.tab+=1;
        while let Some(stmt) = self.parse_stmt() {
            then_branch.push(stmt);
        }
        self.tab-=1;
        
        return ASTNode::ElIf(Box::new(condition), then_branch);

    }

    fn parse_else(&mut self) -> ASTNode {
        self.pos+=1;

        let mut body = Vec::new();
        self.tab+=1;
        while let Some(stmt) = self.parse_stmt() {
            body.push(stmt);
        }
        self.tab-=1;

        return ASTNode::Else(body);
    }

    fn parse_while(&mut self, tab: i32) -> ASTNode {
        self.pos+=1;
        let condition = self.parse_exp().expect("Expected condition for while statement");

        // Store the body statement in a local variable
        let mut body = Vec::new();
        self.tab += 1;
        while let Some(stmt) = self.parse_stmt() {
            body.push(stmt);
        }
        self.tab -= 1;
        
        ASTNode::While(Box::new(condition), body)
    }

    fn parse_print(&mut self, tab: i32) -> ASTNode {
        self.pos+=1;
        if let Some(exp) = self.parse_exp() {
            return ASTNode::Print(Box::new(exp));
        }
        panic!("Failed to parse print statement");
    }

    fn parse_set(&mut self) -> ASTNode {
        self.pos+=1;

        let mut stmt = Vec::new();        

        let var = self.tokens[self.pos].clone();
        if let Token::Identifier(_) = var{
            stmt.push(ASTNode::Var(var.toString()));
            self.pos+=1;
        }else{
            panic!("Failed to parse set statement: need a variable to set");
        }

        if let Some(exp) = self.parse_exp(){
            stmt.push(exp);
            self.pos+=1;
        }else{
            panic!("Failed to parse set statement: need an expression to set the variable to");
        }

        return ASTNode::Set(stmt);
    }

    fn parse_exp(&mut self) -> Option<ASTNode> {
        // Handle expressions (integers, calls, structs, etc.)
        match Some(self.tokens[self.pos].clone()){
            Some(Token::Integer(num)) => {
                self.pos+=1;
                return Some(ASTNode::Integer(num));
            }
            Some(Token::Identifier(name)) => {
                self.pos+=1;
                if let Token::lParen = self.tokens[self.pos].clone(){
                    self.pos+=1;

                    let mut params = Vec::new();
                    while self.tokens[self.pos].clone() != Token::rParen{
                        params.push(self.parse_exp()?);
                    }

                    for x in 0..self.strucs.len(){
                        if let ASTNode::StructDef(struct_name, struct_params, _) = &self.strucs[x]{
                            if name == *struct_name && params.clone().len() == struct_params.len(){
                                if let Token::rParen = self.tokens[self.pos].clone(){
                                    self.pos+=1;
                                    return Some(ASTNode::Struct(struct_name.to_string(), params));
                                }else{
                                    panic!("Failed to parse structure call: missing right parenthesis");
                                }
                            }
                        }
                    }
                    
                    for x in 0..self.funcs.len(){
                        if let ASTNode::FuncDef(func_name, func_params, _, _) = &self.funcs[x]{
                            if name == *func_name && params.clone().len() == func_params.len(){
                                if let Token::rParen = self.tokens[self.pos].clone(){
                                    self.pos+=1;
                                    return Some(ASTNode::Func(func_name.to_string(), params));
                                }else{ 
                                    panic!("Failed to parse function call: missing right parenthesis");
                                }
                            }
                        }
                    }
                    panic!("Failed to parse expression: unknown structure or function call");
                }
                return Some(ASTNode::Var(name));
            }
            Some(Token::Bool(bool)) => {
                self.pos+=1;
                return Some(ASTNode::Bool(bool));
            }
            Some(Token::lParen) => {
                self.pos+=1;
                match Some(self.tokens[self.pos].clone()){
                    Some(Token::Plus) => {
                        self.pos+=1;
                        let exp = Some(ASTNode::AddOrMinusExp(vec![ASTNode::AddOp, self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    Some(Token::Minus) => {
                        self.pos+=1;
                        let exp = Some(ASTNode::AddOrMinusExp(vec![ASTNode::MinusOp, self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    Some(Token::Star) => {
                        self.pos+=1;
                        let exp = Some(ASTNode::MultOrDivExp(vec![ASTNode::MultOp, self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    Some(Token::Div) => {
                        self.pos+=1;
                        let exp =  Some(ASTNode::MultOrDivExp(vec![ASTNode::DivOp, self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    Some(Token::And) => {
                        self.pos+=1;
                        let exp = Some(ASTNode::AndExp(vec![ASTNode::AndOp, self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    Some(Token::Or) => {
                        self.pos+=1;
                        let exp =  Some(ASTNode::OrExp(vec![ASTNode::OrOp, self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    Some(Token::Not) => {
                        self.pos+=1;
                        let exp = Some(ASTNode::NotExp(vec![ASTNode::NotOp, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            return exp;
                        }else{
                            panic!("missing to right parenthesis")
                        }
                    }
                    Some(Token::Equals) | Some(Token::NotEquals) | Some(Token::Less) | Some(Token::Great) | Some(Token::LessEqual) | Some(Token::GreatEqual) => {
                        let token = self.tokens[self.pos].clone();
                        println!("comparison operator {}", token.toString());
			self.pos+=1;

                        let exp = Some(ASTNode::CompExp(vec![ASTNode::CompOp(token.toString()), self.parse_exp()?, self.parse_exp()?]));
                        if let Token::rParen = self.tokens[self.pos].clone(){
                            self.pos+=1;
                            println!("current token {}", self.tokens[self.pos].clone().toString());
                            return exp;
                        }else{
                            panic!("missing the right parenthesis")
                        }
                    }
                    _ => {panic!("not any known expression: {}", self.tokens[self.pos].toString())}
                }
            }
            _ => {panic!("not any known expression: {}", self.tokens[self.pos].toString())}
        }
        None // temp for now
    }
}

