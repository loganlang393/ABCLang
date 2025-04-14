#![allow(warnings)]
mod token;
mod tokenizer;
mod parser; // Include the parser module

use tokenizer::Tokenizer;
use parser::Parser; // Import the Parser struct

fn main() {
    let input = "println(something)";

    let mut tokenizer = Tokenizer::newToken(input);
    let mut parser = Parser::new(tokenizer); // Create a new instance of the parser

    let ast = parser.parse(); // Parse the input
    println!("{:#?}", ast); // Print the parsed AST for verification
}