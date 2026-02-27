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
            right: None,
        }
    }
}

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root_node) = root {
            let (mut left, mut right) = {
                let n = root_node.borrow();
                (n.left.clone(), n.right.clone())
            };
            if let Some(left_node) = left {
                let right_backup = root_node.borrow_mut().right.take();
                root_node.borrow_mut().right = Some(left_node.clone());
                let mut current = left_node.clone();
                loop {
                    let next = current.borrow().right.clone();
                    if let Some(next_node) = next {
                        current = next_node;
                    } else {
                        current.borrow_mut().right = right_backup;
                        break;
                    }
                }
                root_node.borrow_mut().left = None;
            }
            Self::flatten(&mut root_node.borrow_mut().right.clone());
        }
    }
}

fn main() {
    println!("Hello, world!");
}
