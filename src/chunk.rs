
// operation code 
#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
    OpNegate,
    OpAdd,
    OpSubtract,
    OpDivide,
    OpMultiply,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
}

// --
pub struct Chunk {
    arr: Vec<OpCode>,
    arr_line: Vec<usize>,
    const_arr: Vec<Value>,
}

#[derive(Clone)]
pub enum Value {
    Number(f32),
    Boolean(u8),
    Nil,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            arr: Vec::new(),
            const_arr: Vec::new(),
            arr_line: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, code: OpCode, line: usize){
        self.arr.push(code);
        self.arr_line.push(line);
    }

    pub fn get_inst(&self) -> &Vec<OpCode> {
        &self.arr
    }

    pub fn get_const(&self) -> &Vec<Value> {
        &self.const_arr
    }

    pub fn get_line(&self) -> &Vec<usize> {
        &self.arr_line
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        self.const_arr.push(value);
        self.const_arr.len() - 1
    }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) -> usize {

    println!("== {} ==", name);
    
    let cl_arr = chunk.get_inst().iter().zip(chunk.get_line().iter());
    
    let mut last_line: Option<usize> = None;
    for (i, (opc, line)) in cl_arr.enumerate() {
        let line_disp;
        if i > 0 && (*line == last_line.unwrap()) {
            line_disp = format!("   |");
        } else {
            line_disp = format!("{:04}", line);
        }
        
        // default disassembler printout
        print!("{:04} {} {:?}", i, line_disp, opc);
        
        //additional prints
        match opc {
            OpCode::OpConstant(index) => {
                match chunk.get_const()[*index] {
                        Value::Number(n) => print!(" {}", n),
                        Value::Boolean(n) => print!(" {}", n),
                        Value::Nil => print!(" Nil"),
                        _ => (), // potentially more formats to be added
                    }
            }
            _ => (),
        }
        
        println!("");

        last_line = Some(*line);
    }
    
    chunk.get_inst().len()

}
