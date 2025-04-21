#![allow(warnings)]
mod token;
mod tokenizer;
mod parser; // Include the parser module

use tokenizer::Tokenizer;
use parser::Parser; // Import the Parser struct
use std::env; // Import for command line arguments
use std::fs; //import for File Reading


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file).expect("Should be a file");
    println!("Text: {input}");

    let mut tokenizer = Tokenizer::newToken(input.as_str());
    let mut parser = Parser::new(tokenizer); // Create a new instance of the parser

    let ast = parser.parse(); // Parse the input
    println!("{:#?}", ast); // Print the parsed AST for verification
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Failed to parse struct definition: not labeled")]
    fn test_struct_incomplete(){
        let input = "struct";
       
        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }    

    #[test]
    fn test_struct_complete(){
        let input = "struct test()";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    #[should_panic(expected="Failed to parse function definition: not named")]
    fn test_function_incomplete(){
        let input = "func";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_function_complete(){
        let input = "func test() void";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_var_complete(){
        let input = "vardec int test 5";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }
}
