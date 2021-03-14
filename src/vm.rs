
//use crate::scanner;
use crate::compilerf;
use crate::chunk;

pub struct VM {
    curr_ch: chunk::Chunk,
    ip: usize,
    pub stack: Vec<chunk::Value>,
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

        chunk::disassemble_chunk(&self.curr_ch, "CHUNK_DEBUG_DISASSEMBLE"); // DEBUG
        self.run()

        
        //return InterpretResult::InterpretOk
    }

    fn run(&mut self) -> InterpretResult {
        // need to handle this off by 1 error

        loop {
            match self.curr_ch.get_inst()[self.ip] {
                chunk::OpCode::OpReturn => return InterpretResult::InterpretOk,
                chunk::OpCode::OpEqual => {
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m))  if (n == m) => self.stack.push(chunk::Value::Boolean(1)), 
                        (chunk::Value::Boolean(n), chunk::Value::Boolean(m)) if (n == m) => self.stack.push(chunk::Value::Boolean(1)),
                        (chunk::Value::Nil, chunk::Value::Nil) => self.stack.push(chunk::Value::Boolean(1)),
                        (chunk::Value::Number(n), chunk::Value::Number(m))  if (n != m) => self.stack.push(chunk::Value::Boolean(0)), 
                        (chunk::Value::Boolean(n), chunk::Value::Boolean(m)) if (n != m) => self.stack.push(chunk::Value::Boolean(0)),
                        (chunk::Value::Nil, chunk::Value::Number(n)) => self.stack.push(chunk::Value::Boolean(0)),
                        (chunk::Value::Nil, chunk::Value::Boolean(n)) => self.stack.push(chunk::Value::Boolean(0)),
                        (chunk::Value::Number(n), chunk::Value::Nil) => self.stack.push(chunk::Value::Boolean(0)),
                        (chunk::Value::Boolean(n), chunk::Value::Nil) => self.stack.push(chunk::Value::Boolean(0)),
                        _ => {
                            self.runtime_error("Type mismatch for equality operator");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                }
                chunk::OpCode::OpNot => {
                    if let chunk::Value::Boolean(n) = self.stack.pop().unwrap() {
                        match n {
                            0 => self.stack.push(chunk::Value::Boolean(1)),
                            1 => self.stack.push(chunk::Value::Boolean(0)),
                            _ => (),
                        }
                    } else {
                        self.runtime_error("Operand must be a boolean");
                        return InterpretResult::InterpretRuntimeError
                    }
                }
                chunk::OpCode::OpNil => {
                    self.stack.push(chunk::Value::Nil)
                },
                chunk::OpCode::OpTrue => {
                    self.stack.push(chunk::Value::Boolean(1))
                },
                chunk::OpCode::OpFalse => {
                    self.stack.push(chunk::Value::Boolean(0))
                },         
                chunk::OpCode::OpConstant(index) => {
                    self.stack.push(self.curr_ch.get_const()[index].clone());
                },
                chunk::OpCode::OpNegate => {
                    if let chunk::Value::Number(n) = self.stack.pop().unwrap() {
                        self.stack.push(chunk::Value::Number(-n))
                    } else {
                        self.runtime_error("Operand must be a number");
                        return InterpretResult::InterpretRuntimeError
                    }
                }
                chunk::OpCode::OpAdd => { 
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m)) => self.stack.push(chunk::Value::Number(n + m)),
                        _ => {
                            self.runtime_error("Operands must be a numbers");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                },
                chunk::OpCode::OpSubtract =>{
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m)) => self.stack.push(chunk::Value::Number(m - n)),
                        _ => {
                            self.runtime_error("Operands must be a numbers");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                },
                chunk::OpCode::OpDivide =>{
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m)) => self.stack.push(chunk::Value::Number(m / n)),
                        _ => {
                            self.runtime_error("Operands must be a numbers");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                },
                chunk::OpCode::OpMultiply => {
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m)) => self.stack.push(chunk::Value::Number(m * n)),
                        _ => {
                            self.runtime_error("Operands must be a numbers");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                },
                chunk::OpCode::OpLess => {
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m)) if n > m => self.stack.push(chunk::Value::Boolean(1)),
                        (chunk::Value::Number(n), chunk::Value::Number(m)) if n <= m => self.stack.push(chunk::Value::Boolean(0)),
                        _ => {
                            self.runtime_error("Operands must be a numbers");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                },
                chunk::OpCode::OpGreater => {
                    match (self.stack.pop().unwrap(), self.stack.pop().unwrap() ) {
                        (chunk::Value::Number(n), chunk::Value::Number(m)) if n < m => self.stack.push(chunk::Value::Boolean(1)),
                        (chunk::Value::Number(n), chunk::Value::Number(m)) if n >= m => self.stack.push(chunk::Value::Boolean(0)),
                        _ => {
                            self.runtime_error("Operands must be a numbers");
                            return InterpretResult::InterpretRuntimeError
                        }
                    }
                },

            }
            self.ip+=1;
        }
    }

    fn runtime_error(&mut self, message: &str){
        eprint!("{} ", message);
        let mut instruction = self.curr_ch.get_line()[self.ip - 1];

        eprint!("{} in script\n", instruction);
        // reset stack
        self.stack.clear();
    }

    pub fn print_stack(&self) {
        println!("Print stack:");
        for i in self.stack.iter(){
            match i {
                chunk::Value::Number(n) => println!(" {}", n),
                chunk::Value::Boolean(n) => println!(" {}", n),
                chunk::Value::Nil => println!(" Nil"),
                _ => (), // potentially more fields to be added
            }
            
        }
    }

}


