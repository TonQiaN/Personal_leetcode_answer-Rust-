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

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recursion(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(node) = root {
                let (value, left, right) = {
                    let n = node.borrow();
                    (n.val, n.left.clone(), n.right.clone())
                };
                let left_dp = recursion(left);
                let right_dp = recursion(right);
                let dp0 = i32::max(left_dp.0, left_dp.1) + i32::max(right_dp.0, right_dp.1);
                let dp1 = value + left_dp.0 + right_dp.0;
                (dp0, dp1)
            } else {
                (0, 0)
            }
        }
        let (dp0, dp1) = recursion(root);
        i32::max(dp0, dp1)
    }
}

fn main() {
    println!("Hello, world!");
}
