#![allow(warnings)]
mod token;
mod tokenizer;
mod parser; // Include the parser module

use tokenizer::Tokenizer;
use parser::Parser; // Import the Parser struct

fn main() {
    let input = "if ((> 5 4)) \n \t println 5 \n elif ((> 5 3)) \n \t println 3 \n else \n \t println 4";

    let mut tokenizer = Tokenizer::newToken(input);
    let mut parser = Parser::new(tokenizer); // Create a new instance of the parser

    let mut ast = parser.parse();

    println!("{:#?}", ast);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize(){
        let input = "struct func var";

        let mut tokenizer = Tokenizer::newToken(input);

        let mut tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 4);
    }

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
    fn test_func_body(){
        let input = "func test() void \n \t println 5";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_struct_with_func(){
        let input = "struct test() \n \t func test() void \n \t \t println 5";

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

    #[test]
    fn test_var_add_expression(){
        let input = "vardec int test (+ 5 4)";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_var_minus_expression(){
        let input = "vardec int test (- 5 4)";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_if_expression(){
        let input = "if ((> 5 4)) \n \t println 5";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_elif_expression(){
        let input = "if ((> 5 4)) \n \t println 5 \n elif ((> 5 3)) \n \t println 3";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_else_statement(){
        let input = "if ((> 4 5)) \n \t println 4 \n else \n \t println 5";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);
 
        let ast = parser.parse();
    }

    #[test]
    fn test_expression_within_expression(){
        let input = "vardec int test (/(+ 5 4) 3)";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_comparison_expressions(){
        let input = "println (> 4 5) \n println (>= 5 4) \n println (< 5 4) \n println (<= 5 4) \n println (== 5 4) \n println (!= 5 4)";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_return_statement(){
        let input = "return 5";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);
 
        let ast = parser.parse();
    }
 
    #[test]
    fn test_AND_OR_expressions(){
        let input = "return (and (== 5 4) (> 5 4)) \n return (or (== 5 4) (> 5 4))";
        
        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);


        let ast = parser.parse();
    }

    #[test]
    fn test_while_statement(){
        let input = "while (!(> x 5)) \n \t println 5 \n \t println x";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }

    #[test]
    fn test_set_statment(){
        let input = "set x 5";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse();
    }
}
