mod token;
mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let input = "a = 10 + (b * c); print(a);"; // Example input
    let mut tokenizer = Tokenizer::newToken(input);

    loop {
        let token = tokenizer.readToken();
        if token.is_none() {
            break; // Break on none, meaning end of input
        }
        println!("{:?}", token);
    }
}
