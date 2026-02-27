// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

use std::cell::RefCell;
use std::i32;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
            if let Some(node) = root {
                let (val, left, right) = {
                    let node = node.borrow();
                    (node.val, node.left.clone(), node.right.clone())
                };
                let left_bool = is_valid(left, prev);
                if let Some(prev_num) = *prev {
                    if val <= prev_num {
                        return false;
                    }
                }
                *prev = Some(val);
                let right_bool = is_valid(right, prev);
                left_bool && right_bool
            } else {
                true
            }
        }
        let mut prev = None;
        is_valid(root, &mut prev)
    }
}

fn main() {
    println!("Hello, world!");
}
