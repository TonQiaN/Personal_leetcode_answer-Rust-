// #[derive(Debug, PartialEq, Eq)]
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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (None, None) => None,
            (root1, None) =>  root1,
            (None, root2) => root2,
            (Some(node1), Some(node2)) => {
                let (val1, root1l, root1r) = {
                    let node1 = node1.borrow();
                    (node1.val, node1.left.clone(), node1.right.clone())
                };
                let (val2, root2l, root2r) = {
                    let node2 = node2.borrow();
                    (node2.val, node2.left.clone(), node2.right.clone())
                };
                let mut root = TreeNode::new(val1 + val2);
                root.left = Self::merge_trees(root1l, root2l);
                root.right = Self::merge_trees(root1r, root2r);
                Some(Rc::new(RefCell::new(root)))
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
