// 前置記法の処理を抽象化する

use alloc::boxed::Box;

#[derive(Debug,Clone,PartialEq)]
pub enum AppTree<A> {
    Atom(A),
    Apply {
        func: Box<AppTree<A>>,
        arg: Box<AppTree<A>>,
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Parsed<A,M> {
    pub tree: AppTree<A>,
    pub meta: M,
}