#![allow(warnings)]
mod token;
mod tokenizer;
mod parser; // Include the parser module

use tokenizer::Tokenizer;
use parser::Parser; // Import the Parser struct

fn main() {
    let input = "struct test";

    let mut tokenizer = Tokenizer::newToken(input);
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
}
