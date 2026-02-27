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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = i32::MIN;
        fn find_max(root: Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
            if let Some(node) = root {
                let (node_value, node_left, node_right) = {
                    let n = node.borrow();
                    (n.val, n.left.clone(), n.right.clone())
                };
                let left_best = i32::max(find_max(node_left, best), 0);
                let right_best = i32::max(find_max(node_right, best), 0);
                *best = i32::max(*best, node_value + left_best + right_best);
                i32::max(node_value + left_best, node_value + right_best)
            } else {
                return 0;
            }
        }
        find_max(root, &mut best);
        best
    }
}

fn main() {
    println!("Hello, world!");
}
