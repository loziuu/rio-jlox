use std::{
    fs::read_to_string,
    io::{stdin, stdout, Write},
};

use riolox::{
    parser::Parser, printers::AstPrinter, scanner::Scanner, CompilationError, CompilerResult,
};

mod riolox;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        std::process::exit(64)
    }

    if args.len() == 2 {
        args.next().unwrap();
        let file = args.next().unwrap();
        run_file(file);
    } else {
        run_prompt();
    }
}

fn run_file(file: String) {
    let content = read_to_string(file).expect("Failed to read file");
    run(content);
}

fn run_prompt() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        stdout.write_all(b"> ").unwrap();
        stdout.flush().unwrap();

        let mut buffer = "".to_owned();
        let line = stdin.read_line(&mut buffer);

        if line.is_ok() {
            let _ = try_run(buffer);
        } else {
            break;
        }
    }
}

fn run(source: String) {
    let result = try_run(source);
    if result.is_err() {
        std::process::exit(65)
    }
}

fn try_run(source: String) -> CompilerResult {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    // TODO: Remove clone
    let mut parser = Parser::new(tokens.to_vec());

    let expression = parser.parse();

    if expression.is_none() {
        return Err(CompilationError::UndefinedError);
    }

    let printer = AstPrinter {};
    println!("{}", printer.print(&expression.unwrap()));

    Ok(())
}
