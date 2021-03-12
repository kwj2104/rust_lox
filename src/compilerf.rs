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

pub struct ParseRule<'a > {
    prefix: Option<fn(&'a Parser<'a>) -> ()>,
    infix: Option<fn(&'a Parser<'a >) -> ()>,
    prec: u8,
} 

pub struct Parser<'a> {
    pub scanner: scanner::Scanner<'a>,
    pub chunk: chunk::Chunk,
    pub current: scanner::Token,
    pub previous: scanner::Token,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl<'a > Parser<'a> {

    // static HashMap 

    //PARSER CONSTRUCTOR
    fn new(scanner: scanner::Scanner<'a>, chunk: chunk::Chunk) -> Parser {
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
        self.current = self.scanner.next().unwrap();
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
    fn number (&self) {
        // match &self.previous.ttype {
        //     scanner::TokenType::TOKEN_NUMBER(n) => {
        //         let constant = self.chunk.add_const(*n);
        //         self.chunk.write_chunk(chunk::OpCode::OpConstant(constant), self.previous.line);
        //     }
        //     _ => () // Some error handling needed
        // }
    }

    fn grouping(&mut self) {
        self.expression();
    }

    // // EXPRESSION PARSER FUNCTIONS
    fn parse_precedence(&mut self, precedence: u8){
        self.advance()

    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::PREC_ASSIGNMENT);
    }

    fn get_rules(&self, ttype: scanner::TokenType) -> (Option<fn(&'a Parser<'a>) -> ()>, Option<fn() -> ()>, u8){
        match ttype { 
            scanner::TokenType::TOKEN_NUMBER(n)  => (Some(Parser::number), None, Precedence::PREC_NONE),
            _ => (None, None, 0),
            //_ => (),
        }
    }
    


    // EMIT BYTECODE FUNCTIONS - maybe bring back??
    // fn emit_constant(&self, value: &f32) {
    //     let constant = self.chunk.add_const(*value);
    //     self.chunk.write_chunk(chunk::OpCode::OpConstant(constant), self.previous.line);
    //     println!("CONSTANT: {}", value);
    // }
    
}

pub fn compile<'a > (source: &'a str, chunk: chunk::Chunk) -> bool {

    //let str_iter = source.chars();
    let scanner = scanner::Scanner::new(source);
    let parser = Parser::new(scanner, chunk);
    
    return !parser.had_error

}