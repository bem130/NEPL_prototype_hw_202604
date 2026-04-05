use alloc::string::String;
use alloc::boxed::Box;
use crate::ty::Type;

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Unit,
    I32(i32),
    Bool(bool),
    Ident(String),
    Apply {
        func: Box<Expr>,
        input: Box<Expr>,
    },
    Intrinsic(Intrinsic)
}

#[derive(Debug,Clone,PartialEq)]
pub enum Intrinsic {
    Add
}

#[derive(Debug,Clone,PartialEq)]
pub struct TypedExpr {
    pub ex: Expr,
    pub ty: Type, 
}

#[derive(Debug,Clone,PartialEq)]
pub enum ExprAtom {
    Unit,
    Bool(bool),
    I32(i32),
    Ident(String),
    Intrinsic(Intrinsic),
}