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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let (mut left, mut right) = {
                let node = node.borrow();
                (node.left.clone(), node.right.clone())
            };
            let (mut left_count, mut right_count) = (0, 0);
            while let Some(node_left) = left {
                left = node_left.borrow().left.clone();
                left_count += 1;
            }
            while let Some(node_right) = right {
                right = node_right.borrow().right.clone();
                right_count += 1;
            }
            if left_count == right_count {
                return 2 << left_count - 1;
            }
            let (mut left, mut right) = {
                let node = node.borrow();
                (node.left.clone(), node.right.clone())
            };
            return Self::count_nodes(left) + Self::count_nodes(right) + 1;
        }
        0
    }
}


fn main() {
    println!("Hello, world!");
}
