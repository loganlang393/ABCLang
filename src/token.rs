//Tokens
//IDENTIFIER(String)
//INTEGER(int)
//LPAREN
//RPAREN
//PLUS
//MINUS
//STAR
//DIV
//EQUALS
//SEMICOLON
//PRINT

trait Token{
	fn new(&mut self);
	fn equals(&self, t: &dyn Token) -> bool;
	fn toString(&self) -> String;
	fn hashCode(&self) -> u8;
	fn getValue(&self) -> String;
	fn setValue(&mut self, v: String);
}

struct Identifier{
	value: String,
}
struct LParen{
	value: String,
}
struct RParen{
	value: String,
}
struct Plus{
	value: String,
}
struct Minus{
	value: String,
}
struct Star{
	value: String,
}
struct Div{
	value: String,
}
struct Equals{
	value: String,
}
struct Semicolon{
	value: String,
}
struct Print{
	value: String,
}
struct Integer{
	value: String,
}

impl Token for Identifier{
	fn new(&mut self){}
	
        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

	fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
	}

	fn getValue(&self) -> String{
		return self.value.clone();
	}

	fn setValue(&mut self, v: String){
		self.value = v;
	}
}

impl Token for LParen{
	fn new(&mut self){
		self.value = "(".to_string();
	}

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for RParen{
	fn new(&mut self){
		self.value = ")".to_string();
	}

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
		return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Plus{
        fn new(&mut self){
                self.value = "+".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
		return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Minus{
        fn new(&mut self){
                self.value = "-".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Star{
        fn new(&mut self){
                self.value = "*".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
		self.value = v;
        }
}

impl Token for Div{
        fn new(&mut self){
                self.value = "/".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Equals{
        fn new(&mut self){
                self.value = "=".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Semicolon{
        fn new(&mut self){
                self.value = ";".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Print{
        fn new(&mut self){
                self.value = "print".to_string();
        }

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}

impl Token for Integer{
        fn new(&mut self){}

        fn equals(&self, t: &dyn Token) -> bool{
                return t.getValue() == self.value;
        }

        fn toString(&self) -> String{
                return self.value.clone();
        }

        fn hashCode(&self) -> u8{
                return self.value.parse().expect("Invalid Number");
	}

        fn getValue(&self) -> String{
                return self.value.clone();
        }

        fn setValue(&mut self, v: String){
                self.value = v;
        }
}
