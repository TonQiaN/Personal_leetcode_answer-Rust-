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
use std::iter::zip;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(
            left: Option<Rc<RefCell<TreeNode>>>,
            right: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(left_node), Some(right_node)) => {
                    let (left_node_borrow, right_node_borrow) = (left_node.borrow(), right_node.borrow());
                    left_node_borrow.val == right_node_borrow.val
                    && is_mirror(left_node_borrow.left.clone(), right_node_borrow.right.clone())
                    && is_mirror(left_node_borrow.right.clone(), right_node_borrow.left.clone())
                }
                _ => false,
            }
        }

        if let Some(root_node) = root {
            let (left_node, right_node) = {
                let n = root_node.borrow();
                (n.left.clone(), n.right.clone())
            };
            is_mirror(left_node, right_node)
        } else {
            true
        }
    }
}
// use std::cell::RefCell;
// use std::iter::zip;
// use std::rc::Rc;
// impl Solution {
//     pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         fn is_mirror(
//             left: &Option<Rc<RefCell<TreeNode>>>,
//             right: &Option<Rc<RefCell<TreeNode>>>,
//         ) -> bool {
//             match (left, right) {
//                 (None, None) => true,
//                 (Some(left_node_rc), Some(right_node_rc)) => {
//                     let (left_node, right_node) = (left_node_rc.borrow(), right_node_rc.borrow());
//                     left_node.val == right_node.val
//                         && is_mirror(&left_node.left, &right_node.right)
//                         && is_mirror(&left_node.right, &right_node.left)
//                 }
//                 _ => false,
//             }
//         }

//         match root {
//             None => true,
//             Some(node_rc) => {
//                 let node = node_rc.borrow();
//                 is_mirror(&node.left, &node.right)
//             }
//         }
//     }
// }

fn main() {}
