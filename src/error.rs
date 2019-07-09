#[derive(Debug, PartialEq)]
pub enum ParseError {
    TitheNotSet,
    YearNotSet,
    DateError,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct LineError {
    line: usize,
    mode: ParseError,
}
