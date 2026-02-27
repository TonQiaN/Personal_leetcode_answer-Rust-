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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn calculate_max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = root {
                let (left, right) = {
                    let n = node.borrow();
                    (n.left.clone(), n.right.clone())
                };
                1 + i32::max(calculate_max_depth(left), calculate_max_depth(right))
            } else {
                0
            }
        }
        calculate_max_depth(root)
    }
}
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     fn calculate_max_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         if let Some(node) = root {
//             1 + i32::max(
//                 Self::calculate_max_depth(&node.borrow().left),
//                 Self::calculate_max_depth(&node.borrow().right),
//             )
//         } else {
//             0
//         }
//     }
//     pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//         Self::calculate_max_depth(&root)
//     }
// }

fn main() {}
