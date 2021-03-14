mod chunk;
mod vm;
mod scanner;
mod compilerf;

use std::{env, process, io, fs};
use std::error::Error;




fn main() {

    // TEST COMPILER
    //let mut virtual_machine = vm::VM::new();
    // virtual_machine.interpret("1 + 2\0");
    // virtual_machine.print_stack();

    // let test_str = "if 1\0";
    // let scanner = scanner::Scanner::new(test_str);

    // let mut i = 0;
    // for item in scanner {
    //     print!("{} ", i);
    //     println!("type: {:?} start: {} len: {} line: {}", item.ttype, item.start, item.length, item.line);
    //     i += 1;
    // }
    //test bytecode
    //let mut ch = chunk::Chunk::new();


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

// mod scanner_tests {
//     use crate::scanner;

//     #[test]
//     fn keyword() {
//         let test_str = "if if";
//         let mut scanner = scanner::Scanner::new(test_str);


//     }
// }


mod parser_tests {
    use crate::vm;
    use crate::chunk;

    #[test]
    fn addition() {
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1 + 2\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 3.0);
        }
        
    }

    #[test]
    fn subtraction() {
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1 - 2\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, -1.0);
        }
    }

    #[test]
    fn multiply() {
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1 * 2\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 2.0);
        }
    }

    #[test]
    fn divide() {
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1 / 2\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 0.5);
        }
    }

    #[test]
    fn multiple_add(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1 + 2 + 4 + 6\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 13.0);
        }
    }

    #[test]
    fn negate(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("-1\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, -1.0);
        }
    }


    #[test]
    fn add_mult_op(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1 + 2 * 4\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 9.0);
        }
    }


    #[test]
    fn add_mult_paran(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("(1 + 2) * 4\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 12.0);
        }
    }

    #[test]
    fn order_of_ops(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("(-1 + 2) * 3 - -4\0");
        if let chunk::Value::Number(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 7.0);
        }
    }

    #[test]
    fn unary_bang(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("!false\0");
        if let chunk::Value::Boolean(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 1);
        }
    }

    #[test]
    fn equal_bool(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("false==false\0");
        if let chunk::Value::Boolean(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 1);
        }
    }

    #[test]
    fn equal_num(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("1==1\0");
        if let chunk::Value::Boolean(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 1);
        }
    }

    #[test]
    fn greater(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("4 > 3\0");
        if let chunk::Value::Boolean(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 1);
        }
    }

    #[test]
    fn greater_eq(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("4 >= 4\0");
        if let chunk::Value::Boolean(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 1);
        }
    }


    #[test]
    fn comb_bool(){
        let mut virtual_machine = vm::VM::new();
        virtual_machine.interpret("!(5 - 4 > 3 * 2 == !true)\0");
        if let chunk::Value::Boolean(n) = virtual_machine.stack.iter().next().unwrap(){
            assert_eq!(*n, 0);
        }
    }
}