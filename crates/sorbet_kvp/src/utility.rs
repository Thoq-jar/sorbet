const PREFIX: &str = "Sorbet =>";

pub enum SorbetError {
    Syntax,
    SyntaxException,
}

pub fn print_error(kind: SorbetError, message: String) {
    match kind {
        SorbetError::Syntax => println!("{} Syntax error: {:?}", PREFIX, message),
        SorbetError::SyntaxException => println!("{} !EXCEPTION RAISED! Syntax error: {:?}", PREFIX, message),
    }
}

pub fn check_file_extension(sorbet_file_path: String) -> bool {
    sorbet_file_path.ends_with(".srb") || sorbet_file_path.ends_with(".sorbet")
}
