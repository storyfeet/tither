#[derive(Debug, PartialEq, Fail)]
pub enum ParseError {
    #[fail(display = "Tithe Not Set")]
    TitheNotSet,
    #[fail(display = "Year Not Set")]
    YearNotSet,
    #[fail(display = "Date Not Set")]
    DateError,
    //    #[fail(display = "Unknown Error")]
    //    Unknown,
}

#[derive(Debug, PartialEq, Fail)]
#[fail(display = "{} on line {}", mode, line)]
pub struct LineError {
    pub line: usize,
    pub mode: ParseError,
}
