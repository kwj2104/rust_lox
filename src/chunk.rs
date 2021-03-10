
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
}

// --
pub struct Chunk {
    arr: Vec<OpCode>,
    arr_line: Vec<i32>,
    const_arr: Vec<f32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            arr: Vec::new(),
            const_arr: Vec::new(),
            arr_line: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, code: OpCode, line: i32){
        self.arr.push(code);
        self.arr_line.push(line);
    }

    pub fn get_inst(&self) -> &Vec<OpCode> {
        &self.arr
    }

    pub fn get_const(&self) -> &Vec<f32> {
        &self.const_arr
    }

    pub fn get_line(&self) -> &Vec<i32> {
        &self.arr_line
    }

    pub fn add_const(&mut self, value: f32) -> usize {
        self.const_arr.push(value);
        self.const_arr.len() - 1
    }
}

pub fn disassemble_chunk(chunk: &Chunk, name: &str) -> usize {

    println!("== {} ==", name);
    
    let cl_arr = chunk.get_inst().iter().zip(chunk.get_line().iter());
    
    let mut last_line: i32 = -1;
    for (i, (opc, line)) in cl_arr.enumerate() {
        let line_disp;
        if i > 0 && (*line == last_line) {
            line_disp = format!("   |");
        } else {
            line_disp = format!("{:04}", line);
        }
        
        // default disassembler printout
        print!("{:04} {} {:?}", i, line_disp, opc);
        
        //additional prints
        match opc {
            OpCode::OpConstant(index) => print!(" {}", chunk.get_const()[*index]),
            _ => (),
        }
        
        println!("");

        last_line = *line;
    }
    
    chunk.get_inst().len()

}
