

pub struct tokenizer{
    tokes: Vec<char>, //Vec<char> cause I need indexing
    posi: usize,
}

impl tokenizer {
    pub fn newToken(nToken: &str) -> Self{
        tokenizer {
            tokes: nToken.chars().collect(),// converts that string into vec<char>
            posi: 0, //position starts on 0
        }
    }
    
    // Returns index of token
    fn currPosition (&self) -> Option<char> {
        self.tokes.get(self.posi).copied()
    }

    //Skips whitespace
    fn skips (&mut self) {
        let curr = self.currPosition();
        while self.posi < self.tokes.len() {
            if !curr.expect("REASON").is_whitespace(){
                break;
            } 
            self.forwardTokes();
        }
        
    }

    //forwarder
    fn forwardTokes(&mut self) -> Option<char> {
        let curr = self.currPosition();
        self.posi += 1;
        curr
        
    }

    
}
