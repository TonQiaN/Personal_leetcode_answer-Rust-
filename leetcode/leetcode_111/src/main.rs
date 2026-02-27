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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // let Some(root_node) = root else {
        //     return 0;
        // };
        // let (left_node, right_node) = {
        //     let n = root_node.borrow();
        //     (n.left.clone(), n.right.clone())
        // };
        // if left_node.is_none() && right_node.is_none() {
        //     return 1;
        // }
        // let mut l_min = i32::MAX;
        // let mut r_min = i32::MAX;
        // if left_node.is_some() {
        //     l_min = Self::min_depth(left_node);
        // }
        // if right_node.is_some() {
        //     r_min = Self::min_depth(right_node);
        // }
        // i32::min(l_min, r_min) + 1
        if let Some(root) = root {
            let node = root.borrow();
            if node.left.is_none() {
                return 1 + Self::min_depth(node.right.clone());
            }
            if node.right.is_none() {
                return 1 + Self::min_depth(node.left.clone());
            }
            let lm = Self::min_depth(node.left.clone());
            let rm = Self::min_depth(node.right.clone());
            return 1 + lm.min(rm);
        }
        0
    }
}

fn main() {
    println!("Hello, world!");
}
