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
	fn equals(t: Token, &self);
	fn toString(&self);
	fn hashCode(&self);
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
	fn new(identifier: &str) -> Self{
		Self {
			value: identifier.to_string(),
		}
	}
	
        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

	fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
	}
}

impl Token for LParen{
	fn new() -> Self{
		Self {
			value: "(",
		}
	}

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Rparen{
	fn new() -> Self{
		Self{
			value: ")",
		}
	}

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
		return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Plus{
        fn new() -> Self{
                Self{
                        value: "+",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
		return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Minus{
        fn new() -> Self{
                Self{
                        value: "-",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Star{
        fn new() -> Self{
                Self{
                        value: "*",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Div{
        fn new() -> Self{
                Self{
                        value: "/",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Equals{
        fn new() -> Self{
                Self{
                        value: "=",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Semicolon{
        fn new() -> Self{
                Self{
                        value: ";",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Print{
        fn new() -> Self{
                Self{
                        value: "print",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value.as_bytes()[0] as u8;
        }
}

impl Token for Integer{
        fn new() -> Self{
                Self{
                        value: "print",
                }
        }

        fn equals(t: Token, &self) -> bool{
                return t.value == self.value;
        }

        fn toString(&self) -> String{
                return self.value;
        }

        fn hashCode(&self) -> u8{
                return self.value as u8
}
