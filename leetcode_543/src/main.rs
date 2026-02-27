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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;
        fn calculate_depth(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            if let Some(node_rc) = root {
                let (left, right) = {
                    let node = node_rc.borrow();
                    (node.left.clone(), node.right.clone())
                };
                let left_depth = calculate_depth(left, max);
                let right_depth = calculate_depth(right, max);
                if (left_depth + right_depth) > *max {
                    *max = left_depth + right_depth;
                }
                return i32::max(left_depth, right_depth) + 1;
            } else {
                return 0;
            }

        }
        calculate_depth(root, &mut max);
        max
    }
}

fn main() {
    println!("Hello, world!");
}
