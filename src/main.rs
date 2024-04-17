use std::{fs::read_to_string, io::stdin};

use riolox::{scanner::Scanner, CompilerResult};

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

    loop {
        print!("> ");
        let mut buffer = "".to_owned();
        let line = stdin.read_line(&mut buffer);

        if let Ok(_) = line {
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
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn error(line: u32, msg: &str) {
    report(line, "", msg)
}

fn report(line: u32, error: &str, reason: &str) {
    println!("[line {}] Error {}: {}", line, error, reason)
}
