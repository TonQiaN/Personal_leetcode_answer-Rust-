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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut ans = vec![];
        // fn preorder(root:  Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        //     if let Some(node) = root {
        //         let node = node.borrow();
        //         ans.push(node.val);
        //         preorder(node.left.clone(), ans);
        //         preorder(node.right.clone(), ans);
        //     }
        // }
        // preorder(root, &mut ans);
        // ans
        let mut ans = vec![];
        let mut my_stack = vec![];
        my_stack.push(root.clone());
        while let Some(node) = my_stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                ans.push(node.val);
                my_stack.push(node.right.clone());
                my_stack.push(node.left.clone());
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
