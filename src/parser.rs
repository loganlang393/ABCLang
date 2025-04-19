// parser.rs
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use std::process;

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    StructDef(String, Vec<Param>),
    FuncDef(String, Vec<Param>, String, Vec<ASTNode>),
    VarDec(String, String, Box<ASTNode>),
    Var(String),
    Assignment(Box<ASTNode>, Box<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Box<ASTNode>),
    Print(Box<ASTNode>),
    Return(Option<Box<ASTNode>>),
    Block(Vec<ASTNode>),
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
    AddOp,
    MinusOp,
    MultOp,
    DivOp,
    AndOp,
    OrOp,
}

#[derive(Debug)]
pub struct Param {
    var: String, 
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self{
        Parser { tokenizer }
    }

    pub fn parse(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
		
        
        while let Some(token) = self.tokenizer.readToken() {
            match token {
                Token::kwStruct => {
                    nodes.push(self.parse_struct_def());
                }
                Token::kwFunc => {
                    nodes.push(self.parse_func_def());
                }
                Token::kwVarDec => {
                    nodes.push(self.parse_var_dec());
                }
                _ => {
                    // literally everything else
                    if let Some(node) = self.parse_stmt(token.clone()) {
                        nodes.push(node);
                    }
                }
            }
        }

        ASTNode::Program(nodes)
    }

    
// breaks (panics!)
    fn parse_struct_def(&mut self) -> ASTNode {
        println!("Parsing that strucutre!");
        if let Some(Token::Identifier(name)) = self.tokenizer.readToken() {
            println!("Struct name: {}", name);
            let mut params = Vec::new();
            if let Some(Token::lParen) = self.tokenizer.readToken() {
                println!("while loop");
                let mut param = self.tokenizer.readToken();
                while let Some(param) = self.parse_param() {
                    println!("Before Push");
                    params.push(param);
                }
                if matches!(param, Some(Token::rParen)) {
                    return ASTNode::StructDef(name, params);
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
        let param_type = match self.tokenizer.readToken() {
            Some(Token::kwInt) => "int".to_string(),
            Some(Token::kwBool) => "bool".to_string(),
            Some(Token::Identifier(var)) => var,
            _ => return None,
        };

        if let Some(Token::Identifier(var_name)) = self.tokenizer.readToken() {
            return Some(Param { var: var_name});
        }else{
            return None;
        }
    }
    
    

    fn parse_func_def(&mut self) -> ASTNode {
        if let Some(Token::Identifier(name)) = self.tokenizer.readToken() {
            let mut params = Vec::new();

            if let Some(Token::lParen) = self.tokenizer.readToken() {
                while let Some(param) = self.parse_param() {
                    params.push(param);
                }
                if let Some(Token::rParen) = self.tokenizer.readToken() {
                    let ret_type = match self.tokenizer.readToken() {
                        Some(Token::kwVoid) => "void".to_string(),
                        Some(Token::kwInt) => "int".to_string(),
                        Some(Token::kwBool) => "bool".to_string(),
                        _ => panic!("Expected return type"),
                    };

                    let mut body = Vec::new();
                    while let Some(stmt) = self.tokenizer.readToken() {
                        if let Some(node) = self.parse_stmt(stmt.clone()) {
                            body.push(node);
                        }
                    }
                    return ASTNode::FuncDef(name, params, ret_type, body);
                }else{
                    panic!("Failed to parse function definition");
                }
            }else{
                panic!("Failed to parse funcion definition");
            }
        }else{
            panic!("Failed to parse function definition");
        }
    }

    fn parse_var_dec(&mut self) -> ASTNode {
        if let Some(Token::kwVarDec) = self.tokenizer.readToken() {
            let param_type = match self.tokenizer.readToken() {
                Some(Token::kwInt) => "int".to_string(),
                Some(Token::kwBool) => "bool".to_string(),
                _ => panic!("Expected variable type"),
            };

            if let Some(Token::Identifier(var_name)) = self.tokenizer.readToken() {
                if let Some(exp) = self.parse_exp() {
                    return ASTNode::VarDec(var_name, param_type, Box::new(exp));
                }else{
                    panic!("Failed to parse variable declaration");
                }
            }else{
                panic!("Failed to parse variable declaration");
            }
        }else{
            panic!("Failed to parse variable declaration");
        }
    }

// This one actually works, just dies when doing the other parsing
    fn parse_stmt(&mut self, token: Token) -> Option<ASTNode> {
        match token {
            Token::kwReturn => {
                return Some(self.parse_return());
            }
            Token::kwIf => {
                return Some(self.parse_if());
            }
            Token::kwWhile => {
                return Some(self.parse_while());
            }
            Token::kwPrint => {
                return Some(self.parse_print());
            }
            // Add other statements as necessary
            _ => {}
        }

        None
    }

    fn parse_return(&mut self) -> ASTNode {
        let mut exp = None;
        if let Some(token) = self.tokenizer.readToken() {
            if token != Token::rParen {
                exp = Some(Box::new(self.parse_exp().expect("Expected expression")));
            }else{
                panic!("Can't parse return statement")
            }
        }
        ASTNode::Return(exp)
    }

    fn parse_if(&mut self) -> ASTNode {
        let condition = self.parse_exp().expect("Expected condition for if statement");
        let mut then_branch = Vec::new();

        if let Some(Token::lParen) = self.tokenizer.readToken() {
            while let Some(stmt) = self.tokenizer.readToken() {
                if let Some(node) = self.parse_stmt(stmt.clone()) {
                    then_branch.push(node);
                }
            }
            if let Some(Token::rParen) = self.tokenizer.readToken() {
                return ASTNode::If(Box::new(condition), then_branch);
            }
        }

        panic!("Failed to parse if statement");
    }

    fn parse_while(&mut self) -> ASTNode {
        let condition = self.parse_exp().expect("Expected condition for while statement");

        // Store the body statement in a local variable
        let body_stmt = self.tokenizer.readToken().expect("Expected statement for while body");
        let body = self.parse_stmt(body_stmt).expect("Failed to parse while body");
        
        ASTNode::While(Box::new(condition), Box::new(body))
    }

    fn parse_print(&mut self) -> ASTNode {
        if let Some(exp) = self.parse_exp() {
            return ASTNode::Print(Box::new(exp));
        }
        panic!("Failed to parse print statement");
    }

    fn parse_exp(&mut self) -> Option<ASTNode> {
        // Handle expressions (integers, calls, structs, etc.)
        match self.tokenizer.readToken(){
            Some(Token::Integer(num)) => {
                return Some(ASTNode::Integer(num));
            }
            Some(Token::Identifier(name)) => {
                return Some(ASTNode::Var(name));
            }
            Some(Token::Bool(bool)) => {
                return Some(ASTNode::Bool(bool));
            }
            Some(Token::lParen) => {
                match self.tokenizer.readToken(){
                    Some(Token::Plus) => {
                        return Some(ASTNode::AddOrMinusExp(vec![ASTNode::AddOp, self.parse_exp()?, self.parse_exp()?]));
                    }
                    Some(Token::Minus) => {
                        return Some(ASTNode::AddOrMinusExp(vec![ASTNode::MinusOp, self.parse_exp()?, self.parse_exp()?]));
                    }
                    Some(Token::Star) => {
                        return Some(ASTNode::MultOrDivExp(vec![ASTNode::MultOp, self.parse_exp()?, self.parse_exp()?]));
                    }
                    Some(Token::Div) => {
                        return Some(ASTNode::MultOrDivExp(vec![ASTNode::DivOp, self.parse_exp()?, self.parse_exp()?]));
                    }
                    Some(Token::And) => {
                        return Some(ASTNode::AndExp(vec![ASTNode::AndOp, self.parse_exp()?, self.parse_exp()?]));
                    }
                    Some(Token::Or) => {
                        return Some(ASTNode::OrExp(vec![ASTNode::OrOp, self.parse_exp()?, self.parse_exp()?]));
                    }
                    _ => {panic!("not any known expression")}
                }
            }
            _ => {panic!("not any known expression")}
        }
        None // temp for now
    }
}

