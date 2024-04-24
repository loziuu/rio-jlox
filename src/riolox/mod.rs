pub(crate) mod scanner;
pub(crate) mod token;
pub(crate) mod parser;
mod printers;

pub(crate) type CompilerResult = Result<(), CompilationError>;

pub(crate) enum CompilationError {
    UndefinedError,
}

pub fn error(line: usize, msg: &str) {
    report(line, "", msg)
}

pub fn report(line: usize, error: &str, reason: &str) {
    println!("[line {}] Error {}: {}", line, error, reason)
}
