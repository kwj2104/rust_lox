
//use crate::scanner;
//use crate::compiler;
use crate::chunk;

pub struct VM<'a> {
    curr_ch: Option<&'a chunk::Chunk>,
    ip: usize,
    stack: Vec<f32>,
}

#[derive(Debug)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl<'a> VM<'a> {
    pub fn new() ->  VM<'a> {
        VM {curr_ch: None, ip: 0, stack: Vec::new()}
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        //compile(source);
        return InterpretResult::InterpretOk
        //self.curr_ch = Some(ch);
        //self.run()
    }

    fn run(&mut self) -> InterpretResult {
        // need to handle this off by 1 error


        loop {
            match self.curr_ch.unwrap().get_inst()[self.ip] {
                chunk::OpCode::OpReturn => return InterpretResult::InterpretOk,
                chunk::OpCode::OpConstant(index) => {
                    //println!("Constant: {}", self.curr_ch.unwrap().get_const()[index]);
                    self.stack.push(self.curr_ch.unwrap().get_const()[index]);
                },
                chunk::OpCode::OpNegate => {
                    let top = -self.stack.pop().unwrap();
                    self.stack.push(top);
                }
                chunk::OpCode::OpAdd =>{
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                },
                chunk::OpCode::OpSubtract =>{
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                },
                chunk::OpCode::OpDivide =>{
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                },
                chunk::OpCode::OpMultiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                },

            }
            self.ip+=1;
        }
    }

    pub fn print_stack(&self) {
        println!("Print stack:");
        for i in self.stack.iter(){
            println!("{}", i);
        }
    }

}
