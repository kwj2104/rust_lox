
pub struct Token {
    pub ttype: TokenType,
    pub line: usize,
    pub length: usize,
    pub start: usize,
}

impl Token {
    // fn new(type: TokenType) -> Token {
    //     Token {
    //         type: type,
    //         start: scanner.start, 
    //         length: scanner.current - scanner.start,
    //         line: scanner.line,
    //     }
    // }

    // fn new_err(scanner &Scanner) -> Token {
    //     Token {
    //         type: TokenType::TOKEN_ERROR,
    //         start: 0,
    //         length: 0,
    //         line: scanner.line,
    //     }
    // }
}

#[derive(Debug)]
pub enum TokenType {
  // Single-character tokens
  TOKEN_LEFT_PAREN, TOKEN_RIGHT_PAREN,
  TOKEN_LEFT_BRACE, TOKEN_RIGHT_BRACE,
  TOKEN_COMMA, TOKEN_DOT, TOKEN_MINUS, TOKEN_PLUS,
  TOKEN_SEMICOLON, TOKEN_SLASH, TOKEN_STAR,

  // One or two character tokens
  TOKEN_BANG, TOKEN_BANG_EQUAL,
  TOKEN_EQUAL, TOKEN_EQUAL_EQUAL,
  TOKEN_GREATER, TOKEN_GREATER_EQUAL,
  TOKEN_LESS, TOKEN_LESS_EQUAL,

  // Literals
  TOKEN_IDENTIFIER, TOKEN_STRING, TOKEN_NUMBER,

  // Keywords
  TOKEN_AND, TOKEN_CLASS, TOKEN_ELSE, TOKEN_FALSE,
  TOKEN_FOR, TOKEN_FUN, TOKEN_IF, TOKEN_NIL, TOKEN_OR,
  TOKEN_PRINT, TOKEN_RETURN, TOKEN_SUPER, TOKEN_THIS,
  TOKEN_TRUE, TOKEN_VAR, TOKEN_WHILE,

  TOKEN_ERROR(String),
  TOKEN_EOF,
}

pub struct Scanner<'a > {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source_byte: &'a [u8],
}

impl<'a > Scanner<'a > {
    pub fn new(source: &'a str) -> Scanner<'a > {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source_byte: source.as_bytes(),
        }
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source_byte[self.current]
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        Token {
            ttype: ttype,
            start: self.start, 
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn find_string(&mut self) -> Token {
        let mut counter: usize = 1;

        while self.source_byte[self.current + counter] != b'"' {
            match self.advance() {
                b'\n' => { self.line += 1; },
                b'\0' => { return self.make_token(TokenType::TOKEN_ERROR(String::from("Unterminated string"))) }
                _ => { return self.make_token(TokenType::TOKEN_ERROR(String::from("Other error"))) }
            }
            counter += 1;
        }

        self.advance();
        self.make_token(TokenType::TOKEN_STRING)
    }

    fn find_number(&mut self) -> Token {
        let mut dot = false; 
        loop {
            match self.source_byte[self.current] {
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => { self.advance(); },
                b'.' => {
                    if dot == false {
                        dot = true;
                        self.advance();
                    } else {
                        break;
                    }
                },
                _ => break,
            }
        }

        self.make_token(TokenType::TOKEN_NUMBER)
    }
    
    fn find_identifier(&mut self) -> Token {
        loop {
            match self.source_byte[self.current] {
                b'a'..=b'z' | b'A'..=b'Z' | b'_' => { self.advance(); },
                b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => { self.advance(); },
                _ => break,
            }
        }

        self.make_token(TokenType::TOKEN_IDENTIFIER)
    }

    // fn reset(&mut self) {
    //     self.start = 0;
    //     self.current = 0;
    //     line = 1;
    // }
}

impl<'a > Iterator for Scanner<'a > {
    type Item = Token;

    // fn identifier_type(&self) -> Token {
    //     match 
    //     case 'a': return checkKeyword(1, 2, "nd", TOKEN_AND);
    //     case 'c': return checkKeyword(1, 4, "lass", TOKEN_CLASS);
    //     case 'e': return checkKeyword(1, 3, "lse", TOKEN_ELSE);
    //     case 'i': return checkKeyword(1, 1, "f", TOKEN_IF);
    //     case 'n': return checkKeyword(1, 2, "il", TOKEN_NIL);
    //     case 'o': return checkKeyword(1, 1, "r", TOKEN_OR);
    //     case 'p': return checkKeyword(1, 4, "rint", TOKEN_PRINT);
    //     case 'r': return checkKeyword(1, 5, "eturn", TOKEN_RETURN);
    //     case 's': return checkKeyword(1, 4, "uper", TOKEN_SUPER);
    //     case 'v': return checkKeyword(1, 2, "ar", TOKEN_VAR);
    //     case 'w': return checkKeyword(1, 4, "hile", TOKEN_WHILE);
    // }



    fn next(&mut self) -> Option<Token> {
        if self.current == self.source_byte.len(){
            return None
        }

        //check for whitespace
        loop {
            match self.source_byte[self.current] {
                b' ' => {
                    self.advance();
                },
                b'\r' => {
                    self.advance();
                },
                b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.advance();
                    self.line += 1;
                },
                b'/' => {
                    match self.source_byte[self.current + 1] {
                        b'/' => loop { if self.advance() == b'\n' { break;}},
                        _ => break,
                    }
                }
                _ => break,

            }
        }

        self.start = self.current;
        let curr_byte = self.source_byte[self.current];
        self.current += 1;
        match curr_byte {
            //Identifier
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => return Some(self.find_identifier()),

            //String
            b'"' => return Some(self.find_string()), 
            
            //Number
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => return Some(self.find_number()),

            b'(' => return Some(self.make_token(TokenType::TOKEN_LEFT_PAREN)),
            b')' => return Some(self.make_token(TokenType::TOKEN_RIGHT_PAREN)),
            b'{' => return Some(self.make_token(TokenType::TOKEN_LEFT_BRACE)),
            b'}' => return Some(self.make_token(TokenType::TOKEN_RIGHT_BRACE)),
            b';' => return Some(self.make_token(TokenType::TOKEN_SEMICOLON)),
            b',' => return Some(self.make_token(TokenType::TOKEN_COMMA)),
            b'.' => return Some(self.make_token(TokenType::TOKEN_DOT)),
            b'-' => return Some(self.make_token(TokenType::TOKEN_MINUS)),
            b'+' => return Some(self.make_token(TokenType::TOKEN_PLUS)),
            b'/' => return Some(self.make_token(TokenType::TOKEN_SLASH)),
            b'*' => return Some(self.make_token(TokenType::TOKEN_STAR)),
            b'!' => {
                match self.source_byte[self.current] {
                    b'=' => {
                        self.advance();
                        return Some(self.make_token(TokenType::TOKEN_BANG_EQUAL))
                    },
                    _ => return Some(self.make_token(TokenType::TOKEN_BANG))
                }
            },
            b'=' => {
                match self.source_byte[self.current] {
                    b'=' => {
                        self.advance();
                        return Some(self.make_token(TokenType::TOKEN_EQUAL_EQUAL))
                    },
                    _ => return Some(self.make_token(TokenType::TOKEN_EQUAL))
                }
            },
            b'<' => {
                match self.source_byte[self.current]{
                    b'=' => {
                        self.advance();
                        return Some(self.make_token(TokenType::TOKEN_LESS_EQUAL))
                    },
                    _ => return Some(self.make_token(TokenType::TOKEN_LESS))
                }
            },
            b'>' => {
                match self.source_byte[self.current] {
                    b'=' => {
                        self.advance();
                        return Some(self.make_token(TokenType::TOKEN_GREATER_EQUAL))
                    },
                    _ => return Some(self.make_token(TokenType::TOKEN_GREATER))
                }
            },
            b'\0' => return Some(self.make_token(TokenType::TOKEN_EOF)),
            _ => None,
        }
    } 
}


