mod chunk;
mod vm;
mod scanner;
mod compilerf;

use std::{env, process, io, fs};
use std::error::Error;

fn main() {

    // let test_str = "if asdf <> != <= => = \n==\0";
    // //let test_str = "test1 123\0";
    // let mut scanner = scanner::Scanner::new(test_str);

    // let mut i = 0;
    // for item in scanner {
    //     print!("{} ", i);
    //     println!("type: {:?} start: {} len: {} line: {}", item.ttype, item.start, item.length, item.line);
    //     i += 1;
    // }
    //test bytecode
    // let mut ch = chunk::Chunk::new();
    // let mut virtual_machine = vm::VM::new();

    // let args: Vec<String> = env::args().collect(); 

    // if args.len() == 1 {
    //     repl(&virtual_machine);
    // } else if args.len() == 2 {
    //     if let Err(e) = run_file(&virtual_machine, &args[1]) {
    //         eprintln!("Application error: {}", e);
    //     }
    // } else {
    //     eprintln!("Usage: rustlox [path]");
    //     process::exit(1); 
    // }





    // // testing operations
    // let constant = ch.add_const(1.2);
    // ch.write_chunk(chunk::OpCode::OpConstant(constant), 123);
    // let constant2 = ch.add_const(3.4);
    // ch.write_chunk(chunk::OpCode::OpConstant(constant2), 123);    
    
    // ch.write_chunk(chunk::OpCode::OpAdd, 123);

    // let constant3 = ch.add_const(5.4);
    // ch.write_chunk(chunk::OpCode::OpConstant(constant3), 123);

    // ch.write_chunk(chunk::OpCode::OpDivide, 123);

    // ch.write_chunk(chunk::OpCode::OpNegate, 123);

    // ch.write_chunk(chunk::OpCode::OpReturn, 123);
    
    // chunk::disassemble_chunk(&ch, "test_chunk");
    
    // //test VM
    // let mut virtual_machine = vm::VM::new();
    // println!("VM Response: {:?}", virtual_machine.interpret(&ch));
    
    // virtual_machine.print_stack();
}

fn repl(virtual_machine: &vm::VM) {
    loop{
        print!("> ");

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read input");
        println!("test: {}", line);
        //virtual_machine.interpret(line);
    }
}

fn run_file(virtual_machine: &vm::VM, filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    //let result: vm::InterpretResult = virtual_machine.interpret(contents);
    

    // if contents == vm::InterpretResult::InterpretCompileError {
    //     process::exit(70);
    // } else if contents == vm::InterpretResult::InterpretRuntimeError {
    //     process::exit(65);
    // }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::scanner;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn paran() {
        let test_str = "()\0";
        let mut scanner = scanner::Scanner::new(test_str);

        assert_eq!(format!("{:?}", scanner.next().unwrap().ttype), format!("{:?}", scanner::TokenType::TOKEN_LEFT_PAREN));
        assert_eq!(format!("{:?}", scanner.next().unwrap().ttype), format!("{:?}", scanner::TokenType::TOKEN_RIGHT_PAREN));
        assert_eq!(format!("{:?}", scanner.next().unwrap().ttype), format!("{:?}", scanner::TokenType::TOKEN_EOF));
        //assert_eq!(scanner.next().unwrap().ttype, TokenType::TOKEN_RIGHT_PAREN);
    }
}