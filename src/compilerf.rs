use crate::scanner;
use crate::chunk;
use std::collections::HashMap;

struct Precedence;

impl Precedence {
    pub const PREC_NONE: u8 = 0;
    pub const PREC_ASSIGNMENT: u8 = 1;
    pub const PREC_OR: u8 = 2;
    pub const PREC_AND: u8 = 3;
    pub const PREC_EQUALITY: u8 = 4;
    pub const PREC_COMPARISON: u8 = 5;
    pub const PREC_TERM: u8 = 6;
    pub const PREC_FACTOR: u8 = 7;
    pub const PREC_UNARY: u8 = 8;
    pub const PREC_CALL: u8 = 9;
    pub const PREC_PRIMARY: u8 = 10;
}

// pub struct ParseRule<'a > {
//     prefix: Option<fn(&'a Parser<'a>) -> ()>,
//     infix: Option<fn(&'a Parser<'a >) -> ()>,
//     prec: u8,
// } 

pub struct Parser<'a> {
    pub scanner: scanner::Scanner<'a>,
    pub chunk: &'a mut chunk::Chunk,
    pub current: scanner::Token,
    pub previous: scanner::Token,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl<'a> Parser<'a> {

    //PARSER CONSTRUCTOR
    fn new(scanner: scanner::Scanner<'a>, chunk: &'a mut chunk::Chunk) -> Parser<'a > {
        let mut parser = Parser {
            scanner: scanner.clone(),
            chunk: chunk,
            current: scanner.make_token(scanner::TokenType::TOKEN_BOF),
            previous: scanner.make_token(scanner::TokenType::TOKEN_BOF),
            had_error: false,
            panic_mode: false,
        };

        return parser
    }

    // ADVANCE FORWARD 
    fn advance(&mut self) {
        self.previous = self.current.clone();
        self.current = self.scanner.next().unwrap();
    }

    fn consume(&mut self, ttype: scanner::TokenType) {
        if self.current.ttype == ttype {
            self.advance();
            return
        }
    }

    // ERROR HANDLING FUNCTIONS
    fn error_at(&self, token: &scanner::Token, message: &str) {
        eprintln!("{}", token.line);

        match &token.ttype {
            scanner::TokenType::TOKEN_EOF => eprintln!(" at end"),
            scanner::TokenType::TOKEN_ERROR(m) => (),
            _ => eprintln!(" at {} {} ", token.length, token.start)
        }
        if token.ttype == scanner::TokenType::TOKEN_EOF {
            eprintln!(" at end");
        } else if token.ttype == scanner::TokenType::TOKEN_EOF {

        }
    }

    fn error_at_curr(&self, message: &str) {
        self.error_at(&self.current, message);
    }

    fn error(&self, message: &str) {
        self.error_at(&self.previous, message);
    }

    // TOKEN PARSER FUNCTIONS
    //pub fn number (&mut self) {
    fn number (&mut self) {
        match &self.previous.ttype {
            scanner::TokenType::TOKEN_NUMBER(n) => {
                let constant = self.chunk.add_const(chunk::Value::Number(*n));
                self.chunk.write_chunk(chunk::OpCode::OpConstant(constant), self.previous.line);
            }
            _ => () // Some error handling needed
        }
    }

    fn literal (&mut self) {
        match &self.previous.ttype {
            scanner::TokenType::TOKEN_TRUE => self.chunk.write_chunk(chunk::OpCode::OpTrue, self.previous.line),
            scanner::TokenType::TOKEN_FALSE => self.chunk.write_chunk(chunk::OpCode::OpFalse, self.previous.line),
            scanner::TokenType::TOKEN_NIL => self.chunk.write_chunk(chunk::OpCode::OpNil, self.previous.line),
            _ => () // Some error handling needed
        }
    }

    fn binary (&mut self) {
        let operator_type: scanner::TokenType = self.previous.ttype.clone(); //clone necessary?

        let rule = self.get_rules(operator_type.clone());//clone necessary?
        self.parse_precedence(rule.2+1);

        match operator_type {
            scanner::TokenType::TOKEN_PLUS => self.chunk.write_chunk(chunk::OpCode::OpAdd, self.previous.line),
            scanner::TokenType::TOKEN_MINUS => self.chunk.write_chunk(chunk::OpCode::OpSubtract, self.previous.line),
            scanner::TokenType::TOKEN_STAR => self.chunk.write_chunk(chunk::OpCode::OpMultiply, self.previous.line),
            scanner::TokenType::TOKEN_SLASH => self.chunk.write_chunk(chunk::OpCode::OpDivide, self.previous.line),
            scanner::TokenType::TOKEN_BANG_EQUAL => {
                self.chunk.write_chunk(chunk::OpCode::OpEqual, self.previous.line);
                self.chunk.write_chunk(chunk::OpCode::OpNot, self.previous.line);
            }
            scanner::TokenType::TOKEN_EQUAL_EQUAL => self.chunk.write_chunk(chunk::OpCode::OpEqual, self.previous.line),
            scanner::TokenType::TOKEN_GREATER => self.chunk.write_chunk(chunk::OpCode::OpGreater, self.previous.line),
            scanner::TokenType::TOKEN_GREATER_EQUAL => {
                self.chunk.write_chunk(chunk::OpCode::OpLess, self.previous.line);
                self.chunk.write_chunk(chunk::OpCode::OpNot, self.previous.line);
            }
            scanner::TokenType::TOKEN_LESS => self.chunk.write_chunk(chunk::OpCode::OpLess, self.previous.line),
            scanner::TokenType::TOKEN_LESS_EQUAL => {
                self.chunk.write_chunk(chunk::OpCode::OpGreater, self.previous.line);
                self.chunk.write_chunk(chunk::OpCode::OpNot, self.previous.line);
            }
            _ => () // some error handling needed

        }

    }

    fn unary(&mut self) {
        let operator_type: scanner::TokenType = self.previous.ttype.clone(); 
        //self.expression();
        self.parse_precedence(Precedence::PREC_UNARY);

        match operator_type {
            scanner::TokenType::TOKEN_MINUS => self.chunk.write_chunk(chunk::OpCode::OpNegate, self.previous.line),
            scanner::TokenType::TOKEN_BANG => self.chunk.write_chunk(chunk::OpCode::OpNot, self.previous.line),
            _ => (),
        }
    }

    fn grouping(& mut self) {
        self.expression();
        self.consume(scanner::TokenType::TOKEN_RIGHT_PAREN);
    }

    // // EXPRESSION PARSER FUNCTIONS
    fn parse_precedence(&mut self, precedence: u8){
        self.advance();

        let (prefix, _, _) = self.get_rules(self.previous.ttype.clone());
        prefix.unwrap()(self);

        //loop {
        while precedence <= self.get_rules(self.current.ttype.clone()).2 {
            self.advance();
            let infix = self.get_rules(self.previous.ttype.clone()).1;
            infix.unwrap()(self);
        }

    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT);
    }

    fn get_rules(&self, ttype: scanner::TokenType) -> (Option<fn(&mut Parser<'a>) -> ()>, Option<fn(&mut Parser<'a>) -> ()>, u8){
        match ttype { 
            scanner::TokenType::TOKEN_NUMBER(n)  => (Some(Parser::number), None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_MINUS => (Some(Parser::unary), Some(Parser::binary), Precedence::PREC_TERM),
            scanner::TokenType::TOKEN_PLUS => (None, Some(Parser::binary), Precedence::PREC_TERM),
            scanner::TokenType::TOKEN_STAR | scanner::TokenType::TOKEN_SLASH => (None, Some(Parser::binary), Precedence::PREC_FACTOR),
            scanner::TokenType::TOKEN_LEFT_PAREN => (Some(Parser::grouping), None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_RIGHT_PAREN => (None, None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_NIL => (Some(Parser::literal), None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_TRUE => (Some(Parser::literal), None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_FALSE => (Some(Parser::literal), None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_BANG => (Some(Parser::unary), None, Precedence::PREC_NONE),
            scanner::TokenType::TOKEN_BANG_EQUAL => (None, Some(Parser::binary), Precedence::PREC_EQUALITY),
            scanner::TokenType::TOKEN_EQUAL_EQUAL => (None, Some(Parser::binary), Precedence::PREC_EQUALITY),
            scanner::TokenType::TOKEN_GREATER | scanner::TokenType::TOKEN_GREATER_EQUAL | 
            scanner::TokenType::TOKEN_LESS | scanner::TokenType::TOKEN_LESS_EQUAL  => (None, Some(Parser::binary), Precedence::PREC_COMPARISON),
            _ => (None, None, 0),
            //_ => (),
        }
    }
    


    // EMIT BYTECODE FUNCTIONS - maybe bring back??
    fn emit_return(&mut self) {
        self.chunk.write_chunk(chunk::OpCode::OpReturn, self.previous.line);
    }
    
}

pub fn compile<'a > (source: &'a str, chunk: &mut chunk::Chunk) {

    //let str_iter = source.chars();
    //let test_str = "1+2";
    let scanner = scanner::Scanner::new(source);
    let mut parser = Parser::new(scanner, chunk);
    parser.advance();
    parser.expression();
    parser.emit_return();

    //return !parser.had_error -->> NEED TO ADD ERROR HANDLING

}