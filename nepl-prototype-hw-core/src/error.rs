#[derive(Debug,Clone,PartialEq)]
pub enum NEPLErrorKind {
    ParserExcessArity
}

#[derive(Debug,Clone,PartialEq)]
pub struct NEPLError {
    pub kind: NEPLErrorKind,
}