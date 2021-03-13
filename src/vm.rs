
//use crate::scanner;
use crate::compilerf;
use crate::chunk;

pub struct VM {
    curr_ch: chunk::Chunk,
    ip: usize,
    pub stack: Vec<f32>,
}

#[derive(Debug)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl<'a> VM {
    pub fn new() ->  VM {
        VM {curr_ch: chunk::Chunk::new(), ip: 0, stack: Vec::new()}
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        //let mut test_chunk = chunk::Chunk::new();

        compilerf::compile(source, &mut self.curr_ch);
        //
        //self.curr_ch = Some(ch);
        self.run();

        chunk::disassemble_chunk(&self.curr_ch, "CHUNK_DEBUG_DISASSEMBLE"); // DEBUG
        return InterpretResult::InterpretOk
    }

    fn run(&mut self) -> InterpretResult {
        // need to handle this off by 1 error


        loop {
            match self.curr_ch.get_inst()[self.ip] {
                chunk::OpCode::OpReturn => return InterpretResult::InterpretOk,
                chunk::OpCode::OpConstant(index) => {
                    //println!("Constant: {}", self.curr_ch.unwrap().get_const()[index]);
                    self.stack.push(self.curr_ch.get_const()[index]);
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
