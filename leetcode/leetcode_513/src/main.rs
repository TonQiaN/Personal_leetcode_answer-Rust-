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

use std::i32;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = i32::MIN;
        let mut result = 0;
        fn traverse(root: &Rc<RefCell<TreeNode>>, depth: i32, max_depth: &mut i32, result: &mut i32) {
            let (val, left, right) = {
                let node = root.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };
            if left.is_none() && right.is_none() && depth > *max_depth{
                *max_depth = depth;
                *result = val;
            }
            if let Some(left_node) = left {
                traverse(&left_node, depth + 1, max_depth, result);
            }
            if let Some(right_node) = right {
                traverse(&right_node, depth + 1, max_depth, result);
            }
        }
        traverse(&root.unwrap(), 1, &mut max_depth, &mut result);
        result
    }
}

fn main() {
    println!("Hello, world!");
}
