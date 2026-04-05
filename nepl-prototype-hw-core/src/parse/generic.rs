use alloc::vec::Vec;
use core::iter;
use crate::ty::Type;
use alloc::boxed::Box;

use crate::tree::{AppTree,Parsed};

pub fn reduce_one_step_generic<A,M,F>(input: &[Parsed<A,M>], apply_meta: F) -> Option<Vec<Parsed<A,M>>>
    where A: Clone, M: Clone, F: Fn(&M,&M) -> Option<M>, {
        if input.len() < 2 {
            // 処理完了
            return None;
        }
        else {
            // 最初に見つかった適用可能個所だけを縮約する
            for i in 0..(input.len()-1) {
                let left = &input[i];
                let right = &input[i+1];
                // leftがrightを引数として受け取れるかを判定する
                if let Some(new_meta) = apply_meta(&left.meta,&right.meta) {
                    // 該当箇所iで縮約可能であった場合
                    let mut out = Vec::with_capacity(input.len()-1);
                    // 縮約個所より手前はそのままコピー
                    for item in &input[..i] {
                        out.push(item.clone());
                    }
                    // leftとrightを1つのApplyにまとめる
                    out.push(Parsed {
                        tree: AppTree::Apply {
                            func: Box::new(left.tree.clone()),
                            arg: Box::new(right.tree.clone()),
                        },
                        meta: new_meta,
                    });
                    // 縮約個所より後方はそのままコピー
                    for item in &input[(i+2)..] {
                        out.push(item.clone());
                    }
                    // 最初に見つかった個所のみを縮約するのでこれでreturn
                    return Some(out);
                }
            };
            return None
        }
    }

pub fn reduce_all_generic<A,M,F>(input: &[Parsed<A,M>], apply_meta: F) -> Vec<Parsed<A,M>>
    where A: Clone, M: Clone, F: Copy + Fn(&M,&M) -> Option<M>, {
        // 最初の状態は入力
        let mut current = input.to_vec();
        loop {
            match reduce_one_step_generic(&current,apply_meta) {
                Some(next) => {
                    // 適用できたら状態を更新して続行
                    current = next;
                }
                None => {
                    // 適用できなくなったら終了
                    return current;
                }
            }
        }
    }