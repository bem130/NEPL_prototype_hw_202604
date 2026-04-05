use alloc::boxed::Box;
use core::fmt;

#[derive(Debug,Clone,PartialEq)]
pub enum Type {
    Unit,
    I32,
    Bool,
    Function {
        input: Box<Type>,
        result: Box<Type>,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unit => write!(f,"unit"),
            Type::I32 => write!(f,"i32"),
            Type::Bool => write!(f,"bool"),
            Type::Function { input, result } => write!(f, "fn {} {}",input,result),
        }
    }
}


#[test]
fn test_display_type() {
}

#[derive(Debug,Clone,PartialEq)]
pub enum TypeParseMeta {
    Complete,
    Need(usize),
}