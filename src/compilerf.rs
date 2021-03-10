use crate::scanner;

pub fn compile (source: &str) {

    let str_iter = source.chars();
    scanner = scanner::Scanner::new(source);

    let line = -1;

    loop {
        token: Token = scanner.scan_next(str_iter.next());
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