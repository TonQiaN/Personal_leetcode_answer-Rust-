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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut ans = vec![];
        // fn postorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        //     if let Some(node) = root {
        //         let node = node.borrow();
        //         postorder(node.left.clone(), ans);
        //         postorder(node.right.clone(), ans);
        //         ans.push(node.val);
        //     }
        // }
        // postorder(root, &mut ans);
        // ans
        let mut ans = vec![];
        let mut my_stack = vec![];
        my_stack.push(root);
        while let Some(node) = my_stack.pop() {
            if let Some(node) = node {
                let node = node.borrow();
                ans.push(node.val);
                my_stack.push(node.left.clone());
                my_stack.push(node.right.clone());
            }
        }
        ans.reverse();
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
