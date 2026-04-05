use alloc::vec::Vec;
use core::iter;
use crate::ty::Type;

use crate::tree::{AppTree,Parsed};

// pub fn reduce_one_step_generic<A,M,F>(input: &Parsed<A,M>, apply_meta: F) -> Option<Vec<Parsed<A,M>>>
//     where A: Clone, M: Clone, F: Fn(&M,&M) -> Option<N>, {
//         input
//             .windows(2)
//             .enumerate()
//             .find_map(|(i,pair)| {
                
//             })
//     }

// pub fn reduce_all_generic<A,M,F>(input: &Parsed<A,M>, apply_meta: F) -> Vec<Parsed<A,M>>
//     where A: Clone, M: Clone, F: Copy + Fn(&M,&M) -> Option<M>, {
        
//     }

// fn apply_type(func_ty: &Type, arg_ty: &Type) -> Option<Type> {
//     match func_ty {
//         Type::Function { input, result } if **input == *arg_ty => Some((**result).clone()),
//         _ => None,
    