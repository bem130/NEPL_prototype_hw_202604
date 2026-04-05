use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String,ToString};

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