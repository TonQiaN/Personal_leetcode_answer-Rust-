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
      right: None
    }
  }
}

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let (left, right) = {
                let node = node.borrow();
                (node.left.clone(), node.right.clone())
            };
            let left_sum = if let Some(left_node) = left {
                let (ll, lr) = {
                    let lb = left_node.borrow();
                    (lb.left.clone(), lb.right.clone())
                };
                if ll.is_none() && lr.is_none() {
                    left_node.borrow().val
                } else {
                    Self::sum_of_left_leaves(Some(left_node))
                }
            } else {
                0
            };
            let right_sum = Self::sum_of_left_leaves(right);
            left_sum + right_sum
        } else {
            0
        }
    }
    // pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     if let Some(node) = root {
    //         let (left, right) = {
    //             let node = node.borrow();
    //             (node.left.clone(), node.right.clone())
    //         };
    //         if let Some(left_node) = left.clone() {
    //             let (left_left, left_right) = {
    //                 let left_node = left_node.borrow();
    //                 (left_node.left.clone(), left_node.right.clone())
    //             };
    //             if left_left.is_none() && left_right.is_none() {
    //                 return left_node.borrow().val + Self::sum_of_left_leaves(right);
    //             }
    //         } else {
    //             return Self::sum_of_left_leaves(right);
    //         }
    //         Self::sum_of_left_leaves(left) + Self::sum_of_left_leaves(right)
    //     } else {
    //         0
    //     }
    // }
}

fn main() {
    println!("Hello, world!");
}
