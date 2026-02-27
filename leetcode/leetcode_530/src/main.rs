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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_minimum(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>, ans: &mut i32) {
            if let Some(node) = root {
                let (val, left, right) = {
                    let node = node.borrow();
                    (node.val, node.left.clone(), node.right.clone())
                };
                get_minimum(left, prev, ans);
                if let Some(prev_num) = prev {
                    let new_diff = val - *prev_num;
                    if new_diff < *ans {
                        *ans = new_diff;
                    }
                }
                *prev = Some(val);
                get_minimum(right, prev, ans);
            }
        }
        let mut prev = None;
        let mut ans = i32::MAX;
        get_minimum(root, &mut prev, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
