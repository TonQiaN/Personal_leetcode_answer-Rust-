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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let (val, left, right) = {
                let node = node.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };
            if val == target_sum && left.is_none() && right.is_none() {
                true
            } else {
                let left_bool = Self::has_path_sum(left, target_sum - val);
                let right_bool = Self::has_path_sum(right, target_sum - val);
                left_bool || right_bool
            }
        } else {
            false
        }
    }
}

fn main() {
    println!("Hello, world!");
}
