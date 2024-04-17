pub(crate) mod scanner;
pub(crate) mod token;

pub(crate) type CompilerResult = Result<(), CompilationError>;

pub(crate) enum CompilationError {
    UndefinedError,
}



