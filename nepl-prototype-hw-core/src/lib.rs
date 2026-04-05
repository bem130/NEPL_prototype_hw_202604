#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String,ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{self, write};

#[derive(Debug,Clone,PartialEq)]
pub enum TokenKind {
    Ident(String),
    IntLiteral(i32),
    SymbolPercent,
}

pub fn tokenize(input: &str) -> Vec<TokenKind> {
    input.split_whitespace().map(|x| match x {
        "%" => TokenKind::SymbolPercent,
        _ => match x.parse::<i32>() {
            Ok(n) => TokenKind::IntLiteral(n),
            Err(_) => TokenKind::Ident(x.to_string()),
        }
    }).collect()
}

#[test]
fn test_tokenize() {
    assert_eq!(
        tokenize("add 1 2"),
        vec![
            TokenKind::Ident("add".to_string()),
            TokenKind::IntLiteral(1),
            TokenKind::IntLiteral(2),
        ]
    );
}

fn parse() {
    
}

#[derive(Debug,Clone,PartialEq)]
pub enum Type {
    Unit,
    I32,
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
            Type::Function { input, result } => write!(f, "fn {} {}",input,result),
        }
    }
}
#[test]
fn test_display_type() {
}

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Unit,
    I32(i32),
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
pub enum NEPLErrorKind {
    ParserExcessArity
}

#[derive(Debug,Clone,PartialEq)]
pub struct NEPLError {
    pub kind: NEPLErrorKind,
}

// fn reduce_one_step(input: &Vec<TypedExpr>) -> Option<Vec<TypedExpr>> {
    
// }

fn apply_type(func_ty: &Type, arg_ty: &Type) -> Option<Type> {
    match func_ty {
        Type::Function { input, result } if **input == *arg_ty => Some((**result).clone()),
        _ => None,
    }    
}

#[test]
fn test_apply_type() {
    assert_eq!(
        apply_type(
            &Type::Function { input: Box::new(Type::I32), result: Box::new(Type::Unit) },
            &Type::I32,
        ),
        Some(Type::Unit),
    );
    assert_eq!(
        apply_type(
            &Type::Function { input: Box::new(Type::I32), result: Box::new(Type::Unit) },
            &Type::Unit,
        ),
        None,
    );
}