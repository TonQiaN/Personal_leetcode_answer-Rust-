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

use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let (node_val, left, right) = {
                let node = node.borrow();
                (node.val, node.left.clone(), node.right.clone())
            };
            match node_val.cmp(&val) {
                Ordering::Less => Self::search_bst(right, val),
                Ordering::Equal => Some(node),
                Ordering::Greater => Self::search_bst(left, val),
            }
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
