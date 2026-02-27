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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(node) = root else {return None;};
        let (val, left, right) = {
            let node = node.borrow();
            (node.val, node.left.clone(), node.right.clone())
        };
        let p_val = p.clone().unwrap().borrow().val;
        let q_val = q.clone().unwrap().borrow().val;
        let (lo, hi) = (p_val.min(q_val), p_val.max(q_val));
        match (val.cmp(&hi), val.cmp(&lo)) {
            (_, Ordering::Less) => Self::lowest_common_ancestor(right, p, q),
            (Ordering::Greater, _) => Self::lowest_common_ancestor(left, p, q),
            _ => Some(node)            
        }

        // if val < hi && val > lo {
        //     Some(node)
        // } else if val < lo {
        //     Self::lowest_common_ancestor(right, p, q)
        // } else if val > hi {
        //     Self::lowest_common_ancestor(left, p, q)
        // } else {
        //     Some(node)
        // }
    }
}


fn main() {
    println!("Hello, world!");
}
