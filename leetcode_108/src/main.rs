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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            let (left, other) = nums.split_at(mid);
            let (curr, right) = other.split_first().unwrap();
            let mut root = TreeNode::new(*curr);
            root.left = build(left);
            root.right = build(right);
            Some(Rc::new(RefCell::new(root)))
        }
        build(&nums)
    }
}
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
//         fn recuresive_build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
//             if nums.is_empty() {
//                 return None;
//             }
//             let mid = nums.len() / 2;
//             let (left, rest) = nums.split_at(mid);
//             let (curr, right) = rest.split_first().unwrap();
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val: *curr,
//                 left: recuresive_build(left),
//                 right: recuresive_build(right),
//             })))
//         }
//         recuresive_build(&nums)
//     }
// }

fn main() {}
