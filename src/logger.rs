const PREFIX: &str = "Sorbet =>";

pub enum SorbetError {
    Syntax,
}

pub fn print_error(kind: SorbetError, message: String) {
    match kind {
        SorbetError::Syntax => println!("{} Syntax error: {:?}", PREFIX, message),
    }
}
