use crate::parser::parser;
use std::process;

use parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Failed to parse struct definition")]
    fn test_structure_incomplete(){
        let input = "struct";

        let mut tokenizer = Tokenizer::newToken(input);
        let mut parser = Parser::new(tokenizer);

        let ast = parser.parse()
    }
}
