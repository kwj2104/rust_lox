use crate::scanner;

pub fn compile (source: &str) {

    scanner = scanner::Scanner::new(source);

    let line = -1;

    loop {
        token: Token = scanner.next();
        if (token.line != line) {
            println!("{:04}", token.line);
            line = token.line;
        } else {
            println!("  | ");
        }
        if token.type == TokenEOF {
            break;
        }
    }
}