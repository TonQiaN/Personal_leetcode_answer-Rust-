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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                return None;
            }
            let val = *preorder.first().unwrap();
            let i = inorder.iter().position(|&x| x == val).unwrap();
            let mut root_node = TreeNode::new(val);
            root_node.left = build(&preorder[1..1 + i], &inorder[0..i]);
            root_node.right = build(&preorder[1 + i..], &inorder[i + 1..]);
            Some(Rc::new(RefCell::new(root_node)))
        }
        build(&preorder, &inorder)
    }
}