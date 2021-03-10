pub struct Token {
    type: TokenType,
    line: i32,
    length: i32,
    start: usize,
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

pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: i32,
    source: Chars,
}

impl Scanner {
    fn new(source: Chars) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: source,
        }
    }
}

impl Iterator for Scanner {
    type Item = Token;

    fn advance(&self) -> char {
        let result = self.source.next();
        self.current += 1;
        result
    }

    fn make_token(&self, type: TokenType) {
        Token {
            type: type,
            start: self.start, 
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn find_string(&self) -> Token {
        let peek_next = self.source.peek();

        while peek_next != '"' {
            match advance() {
                '\n' => self.line += 1,
                '\0' => make_token(TokenType::TOKEN_ERROR("Unterminated string"))
            }
        }

        advance();
        make_token(TokenType::TOKEN_STRING)
    }

    fn find_number(&self) -> Token {
        let peek_next = self.source.peek();

        while peek_next != '"' {
            match advance() {
                '\n' => self.line += 1,
                '\0' => make_token(TokenType::TOKEN_ERROR("Unterminated string"))
            }
        }

        advance();
        make_token(TokenType::TOKEN_STRING)
    }

    fn next(&mut self) -> Option<Token> {

        //check for whitespace
        loop {
            match self.source.peek() {
                ' ' => {
                    self.advance();
                },
                '\r' => {
                    self.advance();
                },
                '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                },
                '/' => {
                    match self.source.peek().peek() {
                        '/' => loop { if self.advance() == '\n' { self.line += 1; break;}},
                        _ => break,
                    }
                }
                _ => break;

            }
        }

        match self.advance() {
            //String
            '"' => return Some(find_string()), 
            
            //Number
            0..9 => return Some(find_number()),

            '(' => return Some(make_token(TokenType::TOKEN_LEFT_PAREN)),
            ')' => return Some(make_token(TokenType::TOKEN_RIGHT_PAREN)),
            '{' => return Some(make_token(TokenType::TOKEN_LEFT_BRACE))
            '}' => return Some(make_token(TokenType::TOKEN_RIGHT_BRACE)),
            ';' => return Some(make_token(TokenType::TOKEN_SEMICOLON)),
            ',' => return Some(make_token(TokenType::TOKEN_COMMA)),
            '.' => return Some(make_token(TokenType::TOKEN_DOT)),
            '-' => return Some(make_token(TokenType::TOKEN_MINUS)),
            '+' => return Some(make_token(TokenType::TOKEN_PLUS)),
            '/' => return Some(make_token(TokenType::TOKEN_SLASH)),
            '*' => return Some(make_token(TokenType::TOKEN_STAR)),
            '!' => {
                match self.source.peek() {
                    '=' => return Some(make_token(TokenType::TOKEN_BANG_EQUAL)),
                    _ => return Some(make_token(TokenType::TOKEN_BANG))
                }
            },
            '=' => {
                match self.source.peek() {
                    '=' => return Some(make_token(TokenType::TOKEN_EQUAL_EQUAL)),
                    _ => return Some(make_token(TokenType::TOKEN_EQUAL))
                }
            },
            '<' => {
                match self.source.peek() {
                    '=' => return Some(make_token(TokenType::TOKEN_LESS_EQUAL)),
                    _ => return Some(make_token(TokenType::TOKEN_LESS))
                }
            },
            '<' => {
                match self.source.peek() {
                    '=' => return Some(make_token(TokenType::TOKEN_GREATER_EQUAL)),
                    _ => return Some(make_token(TokenType::TOKEN_LESS))
                }
            },
            '\0' => return Some(make_token(TokenType::TOKEN_EOF)),
            _ => None,
        }
    } 
}
