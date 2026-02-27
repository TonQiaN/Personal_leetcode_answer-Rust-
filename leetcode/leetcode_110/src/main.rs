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
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height_or_neg(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let (left, right) = {
                    let node = node.borrow();
                    (node.left.clone(), node.right.clone())
                };
                let lh = height_or_neg(left);
                let rh = height_or_neg(right);
                if (lh - rh).abs() > 1 || lh == -1 || rh == -1{
                    return -1;
                }
                1 + i32::max(lh, rh)
            } else {
                0
            }
        }
        height_or_neg(root) != -1
    }
}

fn main() {
    println!("Hello, world!");
}
